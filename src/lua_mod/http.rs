use mlua::{ExternalResult, Result, Lua, Table};

pub fn mod_http(lua: &Lua) -> Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set("http", lua.create_table_from(vec![
        ("get", lua.create_async_function(get)?),
        ("head", lua.create_async_function(head)?),
    ])?)
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

async fn head<'lua>(lua: &'lua Lua, input: Table<'lua>) -> Result<Table<'lua>> {
    let url: String = input.get("url").into_lua_err()?;
    let resp = reqwest::Client::new()
        .head(&url)
        .send()
        .await
        .and_then(|resp| resp.error_for_status())
        .into_lua_err()?;
    let t = lua.create_table()?;
    t.set("status_code", resp.status().as_u16())?;
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let lua = Lua::new();
        mod_http(&lua).unwrap();
        lua.load(mlua::chunk! {
            local http = require("http")
            local resp = http.get({ url = "https://httpbin.org/get" })
            assert(resp.status_code == 200)
            assert(type(resp.body) == "string")
        }).exec_async().await.unwrap();
    }

    #[tokio::test]
    async fn test_head() {
        let lua = Lua::new();
        mod_http(&lua).unwrap();
        lua.load(mlua::chunk! {
            local http = require("http")
            local resp = http.head({ url = "https://httpbin.org/get" })
            assert(resp.status_code == 200)
        }).exec_async().await.unwrap();
    }
}
