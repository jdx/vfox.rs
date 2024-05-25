use mlua::{IntoLua, Lua, Value};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SdkInfo {
    pub path: PathBuf,
}

impl<'lua> IntoLua<'lua> for SdkInfo {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let table = lua.create_table()?;
        table.set("path", self.path.to_string_lossy().to_string())?;
        Ok(Value::Table(table))
    }
}
