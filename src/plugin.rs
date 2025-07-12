use std::cmp::Ordering;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use mlua::{AsChunk, FromLuaMulti, IntoLua, Lua, Table, Value};
use once_cell::sync::OnceCell;

use crate::config::Config;
use crate::context::Context;
use crate::error::Result;
use crate::hooks::backend_exec_env::{BackendExecEnvContext, BackendExecEnvResponse};
use crate::hooks::backend_install::{BackendInstallContext, BackendInstallResponse};
use crate::hooks::backend_list_versions::{
    BackendListVersionsContext, BackendListVersionsResponse,
};
use crate::hooks::backend_uninstall::{BackendUninstallContext, BackendUninstallResponse};
use crate::metadata::Metadata;
use crate::runtime::Runtime;
use crate::sdk_info::SdkInfo;
use crate::{config, error, lua_mod, VfoxError};

#[derive(Debug)]
pub struct Plugin {
    pub name: String,
    pub dir: PathBuf,
    lua: Lua,
    metadata: OnceCell<Metadata>,
}

impl Plugin {
    pub fn from_dir(dir: &Path) -> Result<Self> {
        if !dir.exists() {
            error!("Plugin directory not found: {:?}", dir);
        }
        let lua = Lua::new();
        lua.set_named_registry_value("plugin_dir", dir.to_path_buf())?;
        Ok(Self {
            name: dir.file_name().unwrap().to_string_lossy().to_string(),
            dir: dir.to_path_buf(),
            lua,
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
        let plugins = plugins
            .iter()
            .filter_map(|p| {
                p.file_name()
                    .and_then(|f| f.to_str())
                    .map(|s| s.to_string())
            })
            .collect();
        Ok(plugins)
    }

    pub fn get_metadata(&self) -> Result<Metadata> {
        Ok(self.load()?.clone())
    }

    pub fn sdk_info(&self, version: String, install_dir: PathBuf) -> Result<SdkInfo> {
        Ok(SdkInfo::new(
            self.get_metadata()?.name.clone(),
            version,
            install_dir,
        ))
    }

    #[cfg(test)]
    pub(crate) fn test(name: &str) -> Self {
        let dir = PathBuf::from("plugins").join(name);
        Self::from_dir(&dir).unwrap()
    }

    pub(crate) fn context(&self, version: Option<String>) -> Result<Context> {
        let ctx = Context {
            args: vec![],
            version,
            // version: "1.0.0".to_string(),
            // runtime_version: "xxx".to_string(),
        };
        Ok(ctx)
    }

    pub(crate) async fn exec_async<'a>(&self, chunk: impl AsChunk<'a>) -> Result<()> {
        self.load()?;
        let chunk = self.lua.load(chunk);
        chunk.exec_async().await?;
        Ok(())
    }

    pub(crate) async fn eval_async<'a, R>(&self, chunk: impl AsChunk<'a>) -> Result<R>
    where
        R: FromLuaMulti,
    {
        self.load()?;
        let chunk = self.lua.load(chunk);
        let result = chunk.eval_async().await?;
        Ok(result)
    }

    pub async fn backend_exec_env(
        &self,
        ctx: BackendExecEnvContext,
    ) -> Result<BackendExecEnvResponse> {
        let metadata = self.get_metadata()?;
        if !metadata.backend_enabled {
            return Err(VfoxError::Error(
                "Plugin does not support backend operations".to_string(),
            ));
        }
        if !metadata.hooks.contains("backend_exec_env") {
            return Err(VfoxError::HookNotFound("backend_exec_env".to_string()));
        }

        trace!("Calling backend_exec_env hook");

        // Set the context in Lua globals
        self.lua.globals().set("CTX", ctx)?;

        let res = self
            .eval_async::<Table>(
                r#"
            local result = PLUGIN.backend_exec_env(CTX)
            return result
        "#,
            )
            .await?;

        let env_vars: Vec<crate::hooks::env_keys::EnvKey> = res.get("env_vars")?;
        Ok(BackendExecEnvResponse { env_vars })
    }

    pub async fn backend_install(
        &self,
        ctx: BackendInstallContext,
    ) -> Result<BackendInstallResponse> {
        let metadata = self.get_metadata()?;
        if !metadata.backend_enabled {
            return Err(VfoxError::Error(
                "Plugin does not support backend operations".to_string(),
            ));
        }
        if !metadata.hooks.contains("backend_install") {
            return Err(VfoxError::HookNotFound("backend_install".to_string()));
        }

        trace!("Calling backend_install hook");

        // Set the context in Lua globals
        self.lua.globals().set("CTX", ctx)?;

        let res = self
            .eval_async::<Table>(
                r#"
            local result = PLUGIN.backend_install(CTX)
            return result
        "#,
            )
            .await?;

        let success: bool = res.get("success")?;
        let message: Option<String> = res.get("message")?;
        Ok(BackendInstallResponse { success, message })
    }

    pub async fn backend_list_versions(
        &self,
        ctx: BackendListVersionsContext,
    ) -> Result<BackendListVersionsResponse> {
        let metadata = self.get_metadata()?;
        if !metadata.backend_enabled {
            return Err(VfoxError::Error(
                "Plugin does not support backend operations".to_string(),
            ));
        }
        if !metadata.hooks.contains("backend_list_versions") {
            return Err(VfoxError::HookNotFound("backend_list_versions".to_string()));
        }

        trace!("Calling backend_list_versions hook");

        // Set the context in Lua globals
        self.lua.globals().set("CTX", ctx)?;

        let res = self
            .eval_async::<Table>(
                r#"
            local result = PLUGIN.backend_list_versions(CTX)
            return result
        "#,
            )
            .await?;

        let versions: Vec<String> = res.get("versions")?;
        Ok(BackendListVersionsResponse { versions })
    }

    pub async fn backend_uninstall(
        &self,
        ctx: BackendUninstallContext,
    ) -> Result<BackendUninstallResponse> {
        let metadata = self.get_metadata()?;
        if !metadata.backend_enabled {
            return Err(VfoxError::Error(
                "Plugin does not support backend operations".to_string(),
            ));
        }
        if !metadata.hooks.contains("backend_uninstall") {
            return Err(VfoxError::HookNotFound("backend_uninstall".to_string()));
        }

        trace!("Calling backend_uninstall hook");

        // Set the context in Lua globals
        self.lua.globals().set("CTX", ctx)?;

        let res = self
            .eval_async::<Table>(
                r#"
            local result = PLUGIN.backend_uninstall(CTX)
            return result
        "#,
            )
            .await?;

        let success: bool = res.get("success")?;
        let message: Option<String> = res.get("message")?;
        Ok(BackendUninstallResponse { success, message })
    }

    fn load(&self) -> Result<&Metadata> {
        self.metadata.get_or_try_init(|| {
            debug!("Getting metadata for {self}");
            set_paths(
                &self.lua,
                &[
                    self.dir.join("?.lua"), //xx
                    self.dir.join("hooks/?.lua"),
                    self.dir.join("lib/?.lua"),
                ],
            )?;

            lua_mod::archiver(&self.lua)?;
            lua_mod::file(&self.lua)?;
            lua_mod::html(&self.lua)?;
            lua_mod::http(&self.lua)?;
            lua_mod::json(&self.lua)?;
            lua_mod::strings(&self.lua)?;
            lua_mod::env(&self.lua)?;

            let metadata = self.load_metadata()?;
            self.set_global("PLUGIN", metadata.clone())?;
            self.set_global("RUNTIME", Runtime::get(self.dir.clone()))?;
            self.set_global("OS_TYPE", config::os())?;
            self.set_global("ARCH_TYPE", config::arch())?;

            let mut metadata: Metadata = metadata.try_into()?;

            // Load file-based hooks
            metadata.hooks = lua_mod::hooks(&self.lua, &self.dir)?;

            // Also check for backend hooks in the PLUGIN table
            self.detect_backend_hooks(&mut metadata, &self.load_metadata()?)?;

            Ok(metadata)
        })
    }

    fn detect_backend_hooks(&self, metadata: &mut Metadata, plugin_table: &Table) -> Result<()> {
        const BACKEND_HOOKS: [&str; 4] = [
            "backend_list_versions",
            "backend_install",
            "backend_exec_env",
            "backend_uninstall",
        ];

        for &hook_name in &BACKEND_HOOKS {
            if let Ok(value) = plugin_table.get::<Value>(hook_name) {
                if !value.is_nil() {
                    metadata.hooks.insert(hook_name);
                }
            }
        }

        Ok(())
    }

    fn set_global<V>(&self, name: &str, value: V) -> Result<()>
    where
        V: IntoLua,
    {
        self.lua.globals().set(name, value)?;
        Ok(())
    }

    fn load_metadata(&self) -> Result<Table> {
        let metadata = self
            .lua
            .load(r#"
                require "metadata"
                return PLUGIN
            "#)
            .eval()?;
        Ok(metadata)
    }
}

fn get_package(lua: &Lua) -> Result<Table> {
    let package = lua.globals().get::<Table>("package")?;
    Ok(package)
}

fn set_paths(lua: &Lua, paths: &[PathBuf]) -> Result<()> {
    let paths = paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(";");

    get_package(lua)?.set("path", paths)?;

    Ok(())
}

impl Display for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq<Self> for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.dir == other.dir
    }
}

impl Eq for Plugin {}

impl PartialOrd for Plugin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Plugin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}
