use mlua::{Lua, Table};

pub fn mod_html(lua: &Lua) -> mlua::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set(
        "htmlparser.voidelements",
        lua.load(include_str!("../../lua/htmlparser/voidelements.lua"))
            .eval::<Table>()?,
    )?;
    loaded.set(
        "htmlparser.ElementNode",
        lua.load(include_str!("../../lua/htmlparser/ElementNode.lua"))
            .eval::<Table>()?,
    )?;
    loaded.set(
        "htmlparser",
        lua.load(include_str!("../../lua/htmlparser.lua"))
            .eval::<Table>()?,
    )?;
    loaded.set(
        "html",
        lua.load(mlua::chunk! {
            local htmlparser = require("htmlparser")
            return {
                parse = function(s)
                    Node = {
                        find = function(self, tag)
                            local nodes = self.node:select(tag)
                            return Node.new(nodes)
                        end,
                        attr = function(self, key)
                            return self.node.attributes[key]
                        end,
                        each = function(self, f)
                            for i, node in ipairs(self.nodes) do
                                f(i - 1, Node.new({node}))
                            end
                        end,
                        first = function(self)
                            return Node.new({self.nodes[1]})
                        end,
                        eq = function(self, idx)
                            local node = self.nodes[idx + 1]
                            return Node.new({node})
                        end,
                        text = function(self)
                            return self.node:getcontent()
                        end
                    }
                    Node.new = function(nodes)
                        return setmetatable({nodes = nodes, node = nodes[1]}, {__index = Node})
                    end
                    local root = htmlparser.parse(s)
                    return Node.new({root})
                end
            }
        })
        .eval::<Table>()?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html() {
        let lua = Lua::new();
        mod_html(&lua).unwrap();
        lua.load(mlua::chunk! {
            local html = require("html")
            local doc = html.parse("<html><body><div id='t2' name='123'>456</div><div foo='bar'>222</div></body></html>")
            local f = doc:find("div"):eq(0)
            local s = doc:find("div"):eq(1)
            assert(s:text() == "222")
            assert(f:text() == "456")

            assert(s:attr("foo") == "bar")

            doc:find("div"):each(function(i, e)
                if i == 0 then
                    assert(e:text() == "456")
                else
                    assert(e:text() == "222")
                end
            end)
        })
            .exec()
            .unwrap();
    }
}
