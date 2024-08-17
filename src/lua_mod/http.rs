use mlua::{ExternalResult, Lua, MultiValue, Result, Table};

pub fn mod_http(lua: &Lua) -> Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set(
        "http",
        lua.create_table_from(vec![
            ("get", lua.create_async_function(get)?),
            ("head", lua.create_async_function(head)?),
            ("download_file", lua.create_async_function(download_file)?),
        ])?,
    )
}

async fn get<'lua>(lua: &'lua Lua, input: Table<'lua>) -> Result<Table<'lua>> {
    let url: String = input.get("url").into_lua_err()?;
    let resp = reqwest::get(&url)
        .await
        .and_then(|resp| resp.error_for_status())
        .into_lua_err()?;
    let t = lua.create_table()?;
    t.set("status_code", resp.status().as_u16())?;
    t.set("headers", get_headers(lua, resp.headers())?)?;
    t.set("body", resp.text().await.into_lua_err()?)?;
    Ok(t)
}

async fn download_file<'lua>(_lua: &'lua Lua, input: MultiValue<'lua>) -> Result<()> {
    let t: &Table = input.iter().next().unwrap().as_table().unwrap();
    let url: String = t.get("url").into_lua_err()?;
    let path: String = input.iter().nth(1).unwrap().to_string()?;
    let resp = reqwest::get(&url).await.into_lua_err()?;
    resp.error_for_status_ref().into_lua_err()?;
    let mut file = tokio::fs::File::create(&path).await.into_lua_err()?;
    let bytes = resp.bytes().await.into_lua_err()?;
    tokio::io::AsyncWriteExt::write_all(&mut file, &bytes)
        .await
        .into_lua_err()?;
    Ok(())
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
    t.set("headers", get_headers(lua, resp.headers())?)?;
    Ok(t)
}

fn get_headers<'lua>(lua: &'lua Lua, headers: &reqwest::header::HeaderMap) -> Result<Table<'lua>> {
    let t = lua.create_table()?;
    for (name, value) in headers.iter() {
        t.set(name.as_str(), value.to_str().into_lua_err()?)?;
    }
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_get() {
        let lua = Lua::new();
        mod_http(&lua).unwrap();
        lua.load(mlua::chunk! {
            local http = require("http")
            local resp = http.get({ url = "https://httpbin.org/get" })
            assert(resp.status_code == 200)
            assert(type(resp.body) == "string")
        })
        .exec_async()
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_head() {
        let lua = Lua::new();
        mod_http(&lua).unwrap();
        lua.load(mlua::chunk! {
            local http = require("http")
            local resp = http.head({ url = "https://httpbin.org/get" })
            print(resp.headers)
            assert(resp.status_code == 200)
            assert(type(resp.headers) == "table")
            assert(resp.headers["content-type"] == "application/json")
            assert(resp.content_length == nil)
        })
        .exec_async()
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore] // TODO: find out why this often fails in CI
    async fn test_download_file() {
        let lua = Lua::new();
        mod_http(&lua).unwrap();
        let path = "test/data/test_download_file.txt";
        lua.load(mlua::chunk! {
            local http = require("http")
            err = http.download_file({
                url = "https://vfox-plugins.lhan.me/index.json",
                headers = {}
            }, $path)
            assert(err == nil, [[must be nil]])
        })
        .exec_async()
        .await
        .unwrap();
        dbg!(fs::read_to_string(path).unwrap());
        // TODO: figure out why this fails on gha
        assert!(fs::read_to_string(path).unwrap().contains("vfox-nodejs"));
        tokio::fs::remove_file(path).await.unwrap();
    }
}
