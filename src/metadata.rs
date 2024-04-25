use mlua::Table;

use crate::Error;
use crate::error::Result;

#[derive(Debug)]
pub struct Metadata {
    pub name: String,
    // pub version: String,
    // pub description: String,
    // pub author: String,
    // pub license: String,
}

impl<'lua> TryFrom<Table<'lua>> for Metadata {
    type Error = Error;
    fn try_from(t: Table<'lua>) -> Result<Self> {
        let name = t.get::<_, String>("name")?;
        // let version = t.get::<_, String>("version")?;
        // let description = t.get::<_, String>("description")?;
        // let author = t.get::<_, String>("author")?;
        // let license = t.get::<_, String>("license")?;
        Ok(Metadata {
            name,
            // version,
            // description,
            // author,
            // license,
        })
    }
}
