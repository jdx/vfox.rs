use mlua::{IntoLua, Lua, LuaSerdeExt, Value};

use crate::error::Result;
use crate::Plugin;

#[derive(Debug)]
pub struct MisePathContext<T: serde::Serialize> {
    pub args: Vec<String>,
    pub options: T,
}

impl Plugin {
    pub async fn mise_path<T: serde::Serialize>(
        &self,
        ctx: MisePathContext<T>,
    ) -> Result<Vec<String>> {
        debug!("[vfox:{}] mise_path", &self.name);
        let path = self
            .eval_async(chunk! {
                require "hooks/mise_path"
                return PLUGIN:MisePath($ctx)
            })
            .await?;
        dbg!(&path);

        Ok(path)
    }
}

impl<'lua, T: serde::Serialize> IntoLua<'lua> for MisePathContext<T> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let table = lua.create_table()?;
        table.set("options", lua.to_value(&self.options)?)?;
        Ok(Value::Table(table))
    }
}
