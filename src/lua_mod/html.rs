use mlua::{Lua, MultiValue, Table};

pub fn mod_html(lua: &Lua) -> mlua::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set("html", lua.create_table_from(vec![
        ("parse", lua.create_function(parse)?),
    ])?)?;
    Ok(())
}

fn parse(lua: &Lua, html: String) -> mlua::Result<Table> {
    let document = scraper::Html::parse_document(&html);
    let _root = document.root_element();
    lua.scope(|scope| {
        let table = lua.create_table()?;
        table.set("find", scope.create_function(|lua, input: MultiValue| {
            let _tag: String = input.iter().nth(1).unwrap().to_string()?;
            let table = lua.create_table()?;
            // TODO: make this work
            // table.set("eq", lua.create_function(move |lua, input: MultiValue| {
            //     let idx: usize = input.iter().nth(1).unwrap().to_string()?.parse().unwrap();
            //     let selector = scraper::Selector::parse(&tag).map_err(|e| mlua::Error::external(e.to_string()))?;
            //     let elements = root.select(&selector).collect::<Vec<_>>();
            //     let element = elements.get(idx).ok_or_else(|| mlua::Error::external("Index out of bounds"))?;
            //     let table = lua.create_table()?;
            //     table.set("text", lua.create_function(move |_, _: ()| {
            //         Ok(element.text().collect::<String>())
            //     })?)?;
            //     Ok(table)
            // })?)?;
            Ok(table)
        })?)?;
        Ok(table)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_html() {
        let lua = Lua::new();
        mod_html(&lua).unwrap();
        lua.load(mlua::chunk! {
            local html = require("html")
            local doc = html.parse("<html><body><div id='t2' name='123'>456</div><div>222</div></body></html>")
            local s = doc:find("div"):eq(1)
            // local f = doc:find("div"):eq(0)
            // local ss = doc:find("div"):eq(2)
            // print(ss:text() == "")
            assert(s:text() == "222")
            // assert(f:text() == "456")
        }).exec().unwrap();
    }
}
