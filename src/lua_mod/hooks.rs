use crate::error::Result;
use mlua::Lua;
use std::path::Path;

pub struct HookFunc {
    _name: &'static str,
    required: bool,
    filename: &'static str,
}

#[rustfmt::skip]
const HOOK_FUNCS: [HookFunc; 7] = [
    HookFunc { _name: "Available", required: true, filename: "available" },
    HookFunc { _name: "PreInstall", required: true, filename: "pre_install" },
    HookFunc { _name: "EnvKeys", required: true, filename: "env_keys" },
    HookFunc { _name: "PostInstall", required: false, filename: "post_install" },
    HookFunc { _name: "PreUse", required: false, filename: "pre_use" },
    HookFunc { _name: "ParseLegacyFile", required: false, filename: "parse_legacy_file" },
    HookFunc { _name: "PreUninstall", required: false, filename: "pre_uninstall" },
];

pub fn mod_hooks(lua: &Lua, root: &Path) -> Result<()> {
    for hook in &HOOK_FUNCS {
        let hook_path = root.join("hooks").join(format!("{}.lua", hook.filename));
        if hook_path.exists() {
            lua.load(hook_path).exec()?;
        } else if hook.required {
            return Err(format!("Required hook '{}' not found", hook.filename).into());
        }
    }
    Ok(())
}
