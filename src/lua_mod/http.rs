use mlua::{ExternalResult, Result, Lua, Table};

pub fn mod_http(lua: &Lua) -> Result<Table> {
    lua.create_table_from(vec![
        ("get", lua.create_async_function(get)?),
    ])
}

async fn get<'lua>(lua: &'lua Lua, input: Table<'lua>) -> Result<Table<'lua>> {
    let url: String = input.get("url").into_lua_err()?;
    let resp = reqwest::get(&url)
        .await
        .and_then(|resp| resp.error_for_status())
        .into_lua_err()?;
    let t = lua.create_table()?;
    t.set("status_code", resp.status().as_u16())?;
    t.set("body", resp.text().await.into_lua_err()?)?;
    Ok(t)
}
