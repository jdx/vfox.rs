use mlua::{ExternalResult, Result, Lua, Table, Value, LuaSerdeExt};

pub fn mod_json(lua: &Lua) -> Result<Table> {
    lua.create_table_from(vec![
        ("encode", lua.create_function(encode)?),
        ("decode", lua.create_function(decode)?),
    ])
}

fn encode(_lua: &Lua, value: Value) -> Result<String> {
    serde_json::to_string(&value).into_lua_err()
}

fn decode(lua: &Lua, value: String) -> Result<Value> {
    let value: serde_json::Value = serde_json::from_str(&value).into_lua_err()?;
    lua.to_value(&value)
}
