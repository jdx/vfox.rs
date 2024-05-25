use mlua::{IntoLua, Lua, Value};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SdkInfo {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
}

impl SdkInfo {
    pub fn new(name: String, version: String, path: PathBuf) -> Self {
        Self {
            name,
            version,
            path,
        }
    }
}

impl<'lua> IntoLua<'lua> for SdkInfo {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let table = lua.create_table()?;
        table.set("name", self.name)?;
        table.set("version", self.version)?;
        table.set("path", self.path.to_string_lossy().to_string())?;
        Ok(Value::Table(table))
    }
}
