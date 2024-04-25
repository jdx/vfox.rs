use mlua::{UserData, UserDataFields};

#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) version: Option<String>,
    // pub(crate) runtime_version: String,
}

impl UserData for Context {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("version", |_, t| Ok(t.version.clone()));
        // fields.add_field_method_get("runtimeVersion", |_, t| Ok(t.runtime_version.clone()));
    }
}