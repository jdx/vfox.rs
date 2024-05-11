use mlua::prelude::LuaError;
use mlua::{FromLua, Lua, Value};

use crate::error::Result;
use crate::Plugin;

impl Plugin {
    #[tokio::main(flavor = "current_thread")]
    pub async fn available(&self) -> Result<Vec<AvailableVersion>> {
        self.available_async().await
    }

    pub async fn available_async(&self) -> Result<Vec<AvailableVersion>> {
        let ctx = self.context(None)?;
        let available = self
            .eval_async(chunk! {
                require "hooks/available"
                return PLUGIN:Available($ctx)
            })
            .await?;

        Ok(available)
    }
}

#[derive(Debug)]
pub struct AvailableVersion {
    pub version: String,
    pub note: Option<String>,
    // pub addition: Option<Table<'lua>>,
}

impl<'lua> FromLua<'lua> for AvailableVersion {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> std::result::Result<Self, LuaError> {
        match value {
            Value::Table(table) => {
                // TODO: try to default this to an empty table or something
                // let addition = table.get::<_, Option<Table>>("addition")?;
                Ok(AvailableVersion {
                    version: table.get::<_, String>("version")?,
                    note: table.get::<_, Option<String>>("note")?,
                    // addition,
                })
            }
            _ => panic!("Expected table"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Plugin;

    #[test]
    fn dummy() {
        let versions = run("vfox-dummy");
        assert_debug_snapshot!(versions, @r###"
        [
            "1.0.0",
            "1.0.1",
        ]
        "###);
    }

    #[tokio::test]
    async fn dummy_async() {
        let versions = run_async("vfox-dummy").await;
        assert_debug_snapshot!(versions, @r###"
        [
            "1.0.0",
            "1.0.1",
        ]
        "###);
    }

    #[tokio::test]
    async fn nodejs_async() {
        let versions = run_async("vfox-nodejs").await;
        assert!(versions.contains(&"20.0.0".to_string()));
    }

    fn run(plugin: &str) -> Vec<String> {
        let p = Plugin::test(plugin);
        let r = p.available().unwrap();
        r.iter().map(|v| v.version.clone()).collect()
    }

    async fn run_async(plugin: &str) -> Vec<String> {
        let p = Plugin::test(plugin);
        let r = p.available_async().await.unwrap();
        r.iter().map(|v| v.version.clone()).collect()
    }
}
