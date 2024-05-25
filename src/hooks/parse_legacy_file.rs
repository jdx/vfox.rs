use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, MultiValue, Value};
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::plugin::AppData;
use crate::Plugin;

#[derive(Debug)]
pub struct LegacyFileContext {
    pub filepath: PathBuf,
}

#[derive(Debug)]
pub struct ParseLegacyFileResponse {
    pub version: Option<String>,
}

impl Plugin {
    pub async fn parse_legacy_file(&self, legacy_file: &Path) -> Result<ParseLegacyFileResponse> {
        let ctx = LegacyFileContext {
            filepath: legacy_file.to_path_buf(),
        };
        let legacy_file_response = self
            .eval_async(chunk! {
                require "hooks/available"
                require "hooks/parse_legacy_file"
                return PLUGIN:ParseLegacyFile($ctx)
            })
            .await?;

        Ok(legacy_file_response)
    }
}

impl<'lua> IntoLua<'lua> for LegacyFileContext {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let table = lua.create_table()?;
        table.set("filepath", self.filepath.to_string_lossy().to_string())?;
        table.set(
            "getInstalledVersions",
            lua.create_async_function(move |lua, _input: MultiValue| async {
                let app_data = lua.app_data_ref::<AppData>().unwrap();
                Ok(Plugin::from_dir(app_data.plugin_dir.as_path())
                    .map_err(|e| LuaError::RuntimeError(e.to_string()))?
                    .available_async()
                    .await
                    .map_err(|e| LuaError::RuntimeError(e.to_string()))?
                    .into_iter()
                    .map(|v| v.version)
                    .collect::<Vec<String>>())
            })?,
        )?;
        Ok(Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for ParseLegacyFileResponse {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> std::result::Result<Self, LuaError> {
        match value {
            Value::Table(table) => Ok(ParseLegacyFileResponse {
                version: table.get::<_, Option<String>>("version")?,
            }),
            _ => panic!("Expected table"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vfox;

    #[tokio::test]
    async fn test_parse_legacy_file_nodejs() {
        let vfox = Vfox::test();
        let response = vfox
            .parse_legacy_file("vfox-nodejs", Path::new("test/data/.node-version"))
            .await
            .unwrap();
        let out = format!("{:?}", response);
        assert_snapshot!(out, @r###"ParseLegacyFileResponse { version: Some("20.0.0") }"###);
    }
}
