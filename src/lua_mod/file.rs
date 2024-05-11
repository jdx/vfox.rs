use std::path::Path;
use std::os::unix::fs::symlink as _symlink;
use mlua::{ExternalResult, Lua, MultiValue, Table};
use crate::error::Result;

pub fn mod_file(lua: &Lua) -> Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    Ok(loaded.set("file", lua.create_table_from(vec![
        ("symlink", lua.create_async_function(symlink)?),
    ])?)?)
}

async fn symlink<'lua>(_lua: &'lua Lua, input: MultiValue<'lua>) -> mlua::Result<()> {
    let input: Vec<String> = input.into_iter().map(|v| v.to_string()).collect::<mlua::Result<_>>()?;
    let src = Path::new(&input[0]);
    let dst = Path::new(&input[1]);
    _symlink(src, dst).into_lua_err()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_symlink() {
        let _ = fs::remove_file("/tmp/test_symlink_dst");
        let lua = Lua::new();
        mod_file(&lua).unwrap();
        lua.load(mlua::chunk! {
            local file = require("file")
            file.symlink("/tmp/test_symlink_src", "/tmp/test_symlink_dst")
        }).exec().unwrap();
        assert_eq!(fs::read_link("/tmp/test_symlink_dst").unwrap(), Path::new("/tmp/test_symlink_src"));
        fs::remove_file("/tmp/test_symlink_dst").unwrap();
    }
}