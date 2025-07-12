use mlua::Table;
use std::collections::BTreeSet;

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
    pub hooks: BTreeSet<&'static str>,
    pub backend_enabled: bool,
    pub backend_name: Option<String>,
}

impl TryFrom<Table> for Metadata {
    type Error = VfoxError;
    fn try_from(t: Table) -> Result<Self> {
        let legacy_filenames = t
            .get::<Option<Vec<String>>>("legacyFilenames")?
            .unwrap_or_default();
        let backend_enabled = t.get::<Option<bool>>("backendEnabled")?.unwrap_or(false);
        let backend_name = t.get::<Option<String>>("backendName")?;

        Ok(Metadata {
            name: t.get("name")?,
            legacy_filenames,
            version: t.get("version")?,
            description: t.get("description")?,
            author: t.get("author")?,
            license: t.get("license")?,
            homepage: t.get("homepage")?,
            hooks: Default::default(),
            backend_enabled,
            backend_name,
        })
    }
}
