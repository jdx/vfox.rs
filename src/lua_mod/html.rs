use mlua::{AnyUserDataExt, Lua, MultiValue, Table};
use serde::{Deserialize, Serialize};

pub fn mod_html(lua: &Lua) -> mlua::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set(
        "html",
        lua.load(include_str!("../../lua/html.lua"))
            .eval::<Table>()?,
    )?;
    Ok(())
}

// #[derive(Serialize, Deserialize)]
// struct HtmlUserdata {
//     html: String,
// }
//
// impl mlua::UserData for HtmlUserdata {}
//
// fn parse(lua: &Lua, html: String) -> mlua::Result<Table> {
//     lua.scope(|scope| {
//         let ud = scope.create_userdata(HtmlUserdata { html })?;
//         let document = scraper::Html::parse_document(ud.gej);
//         let root = document.root_element();
//         let table = lua.create_table()?;
//         table.set(
//             "find",
//             scope.create_function(|lua, input: MultiValue| {
//                 let tag: String = input.iter().nth(1).unwrap().to_string()?;
//                 let table = lua.create_table()?;
//                 // TODO: make this work
//                 table.set(
//                     "eq",
//                     lua.create_function(move |lua, input: MultiValue| {
//                         let idx: usize = input.iter().nth(1).unwrap().to_string()?.parse().unwrap();
//                         let selector = scraper::Selector::parse(&tag)
//                             .map_err(|e| mlua::Error::external(e.to_string()))?;
//                         let elements = root.select(&selector).collect::<Vec<_>>();
//                         let element = elements
//                             .get(idx)
//                             .ok_or_else(|| mlua::Error::external("Index out of bounds"))?
//                             .clone();
//                         let table = lua.create_table()?;
//                         table.set(
//                             "text",
//                             lua.create_function(move |_, _: ()| {
//                                 Ok(element.text().collect::<String>())
//                             })?,
//                         )?;
//                         Ok(table)
//                     })?,
//                 )?;
//                 Ok(table)
//             })?,
//         )?;
//         Ok(table)
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html() {
        let lua = Lua::new();
        mod_html(&lua).unwrap();
        let sample = include_str!("../../test/data/sample.html");
        lua.load(mlua::chunk! {
            local html = require("html")
            local doc = html.parse($sample)

            local chapters = doc:find("ol.chapters > li")

            for i, chapter in ipairs(chapters) do
                print(i, chapter:text())
            end

            print(chapters:eq(0))
            print(chapters:eq(1))
            // assert(chapters:eq(0):find("a"):eq(0):text() == "Chapter 1")
        })
            .exec()
            .unwrap();
    }
}
