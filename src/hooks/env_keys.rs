use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, Value};
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::error::Result;
use crate::sdk_info::SdkInfo;
use crate::Plugin;

#[derive(Debug)]
pub struct EnvKey {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct EnvKeysContext {
    pub version: String,
    pub path: PathBuf,
    pub sdk_info: BTreeMap<String, SdkInfo>,
}

impl Plugin {
    pub async fn env_keys(&self, ctx: EnvKeysContext) -> Result<Vec<EnvKey>> {
        let env_keys = self
            .eval_async(chunk! {
                require "hooks/env_keys"
                return PLUGIN:EnvKeys($ctx)
            })
            .await?;

        Ok(env_keys)
    }
}

impl<'lua> IntoLua<'lua> for EnvKeysContext {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let table = lua.create_table()?;
        table.set("version", self.version)?;
        table.set("path", self.path.to_string_lossy().to_string())?;
        let sdk_info = lua.create_table()?;
        for (k, v) in self.sdk_info {
            sdk_info.set(k, v)?;
        }
        table.set("sdk_info", sdk_info)?;
        Ok(Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for EnvKey {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> std::result::Result<Self, LuaError> {
        match value {
            Value::Table(table) => Ok(EnvKey {
                key: table.get::<_, String>("key")?,
                value: table.get::<_, String>("value")?,
            }),
            _ => panic!("Expected table"),
        }
    }
}
