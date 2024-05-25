use mlua::Table;

use crate::error::Result;
use crate::error::VfoxError;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub legacy_filenames: Vec<String>,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
}

impl<'lua> TryFrom<Table<'lua>> for Metadata {
    type Error = VfoxError;
    fn try_from(t: Table<'lua>) -> Result<Self> {
        let legacy_filenames = t
            .get::<_, Option<Vec<String>>>("legacyFilenames")?
            .unwrap_or_default();
        Ok(Metadata {
            name: t.get("name")?,
            legacy_filenames,
            version: t.get("version")?,
            description: t.get("description")?,
            author: t.get("author")?,
            license: t.get("license")?,
            homepage: t.get("homepage")?,
        })
    }
}
