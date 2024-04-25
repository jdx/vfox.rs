use std::path::{Path, PathBuf};

use mlua::{AsChunk, ExternalResult, FromLuaMulti, IntoLua, Lua, LuaSerdeExt, Table, Value};
use once_cell::sync::OnceCell;
use crate::config::Config;

use crate::context::Context;
use crate::error::Result;
use crate::metadata::Metadata;
use crate::runtime::Runtime;

pub struct Plugin {
    pub dir: PathBuf,
    lua: Lua,
    metadata: OnceCell<Metadata>,
}

impl Plugin {
    pub fn from_dir(dir: &Path) -> Result<Self> {
        Ok(Self {
            dir: dir.to_path_buf(),
            lua: Lua::new(),
            metadata: OnceCell::new(),
        })
    }

    pub fn from_name(name: &str) -> Result<Self> {
        let dir = Config::get().plugin_dir.join(name);
        Self::from_dir(&dir)
    }

    pub fn list() -> Result<Vec<String>> {
        let config = Config::get();
        if !config.plugin_dir.exists() {
            return Ok(vec![]);
        }
        let plugins = xx::file::ls(&config.plugin_dir)?;
        let plugins = plugins.iter()
            .filter_map(|p| p.file_name().and_then(|f| f.to_str()).map(|s| s.to_string()))
            .collect();
        Ok(plugins)
    }

    #[cfg(test)]
    pub(crate) fn test(name: &str) -> Self {
        let dir = PathBuf::from("plugins").join(name);
        Self::from_dir(&dir).unwrap()
    }

    pub(crate) fn context(&self, version: Option<String>) -> Result<Context> {
        let ctx = Context {
            version,
            // version: "1.0.0".to_string(),
            // runtime_version: "xxx".to_string(),
        };
        Ok(ctx)
    }

    pub(crate) async fn exec_async<'lua, 'a>(&'lua self, chunk: impl AsChunk<'lua, 'a>) -> Result<()> {
        self.load()?;
        let chunk = self.lua.load(chunk);
        chunk.exec_async().await?;
        Ok(())
    }

    pub(crate) async fn eval_async<'lua, 'a, R>(&'lua self, chunk: impl AsChunk<'lua, 'a>) -> Result<R>
        where
            R: FromLuaMulti<'lua> + 'lua
    {
        self.load()?;
        let chunk = self.lua.load(chunk);
        let result = chunk.eval_async().await?;
        Ok(result)
    }

    fn load(&self) -> Result<&Metadata> {
        self.metadata.get_or_try_init(|| {
            set_paths(&self.lua, &[
                self.dir.join("?.lua"),//xx
                self.dir.join("hooks/?.lua"),
                self.dir.join("lib/?.lua"),
            ])?;

            self.set_mod("json", self.mod_json()?)?;
            self.set_mod("http", self.mod_http()?)?;

            let metadata = self.load_metadata()?;
            self.set_global("PLUGIN", metadata.clone())?;
            self.set_global("RUNTIME", Runtime::get())?;

            metadata.try_into()
        })
    }


    fn set_global<'lua, V>(&'lua self, name: &str, value: V) -> Result<()>
        where
            V: IntoLua<'lua>
    {
        self.lua.globals().set(name, value)?;
        Ok(())
    }

    fn load_metadata(&self) -> Result<Table> {
        let metadata = self.lua.load(chunk! {
            require "metadata"
            return PLUGIN
        }).eval()?;
        Ok(metadata)
    }

    fn mod_json(&self) -> Result<Table> {
        let encode = self.lua.create_function(|_, value: Value| {
            serde_json::to_string(&value).into_lua_err()
        })?;
        let decode = self.lua.create_function(|lua, value: String| {
            let value: serde_json::Value = serde_json::from_str(&value).into_lua_err()?;
            Ok(lua.to_value(&value))
        })?;
        let t = self.lua.create_table_from(vec![
            ("encode", encode),
            ("decode", decode),
        ])?;
        Ok(t)
    }

    fn mod_http(&self) -> Result<Table> {
        let get = self.lua.create_async_function(|lua, input: Table| async move {
            let url: String = input.get("url").into_lua_err()?;
            let resp = reqwest::get(&url)
                .await
                .and_then(|resp| resp.error_for_status())
                .into_lua_err()?;
            let t = lua.create_table()?;
            t.set("status_code", resp.status().as_u16())?;
            t.set("body", resp.text().await.into_lua_err()?)?;
            Ok(t)
        })?;
        let t = self.lua.create_table_from(vec![
            ("get", get),
        ])?;
        Ok(t)
    }

    fn set_mod(&self, name: &str, m: Table) -> Result<()> {
        let loaded = get_package(&self.lua)?.get::<_, Table>("loaded")?;
        loaded.set(name, m)?;
        Ok(())
    }
}

fn get_package(lua: &Lua) -> Result<Table> {
    let package = lua.globals().get::<_, Table>("package")?;
    Ok(package)
}

fn set_paths(lua: &Lua, paths: &[PathBuf]) -> Result<()> {
    let paths = paths.iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(";");

    get_package(lua)?.set("path", paths)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list() {
        Plugin::list().unwrap();
    }
}
