use mlua::prelude::*;

pub fn available() -> Vec<String> {
    let lua = Lua::new();

    let map_table = lua.create_table().unwrap();
    map_table.set(1, "one").unwrap();
    map_table.set("two", 2).unwrap();

    lua.globals().set("map_table", map_table).unwrap();

    lua.load("for k,v in pairs(map_table) do print(k,v) end").exec().unwrap();

    Vec::from(["one".to_string(), "two".to_string()])
}
