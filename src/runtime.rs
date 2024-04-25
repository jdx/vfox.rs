use std::env::consts::{ARCH, OS};
use std::sync::{Mutex, MutexGuard};
use mlua::{UserData, UserDataFields};

#[derive(Debug, Clone)]
pub(crate) struct Runtime {
    pub(crate) os: String,
    pub(crate) arch: String,
    // pub(crate) version: String,
}

static RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);

impl Runtime {
    pub(crate) fn get() -> Self {
        Self::_get().as_ref().unwrap().clone()
    }

    fn _get() -> MutexGuard<'static, Option<Runtime>> {
        let mut runtime = RUNTIME.lock().unwrap();
        if runtime.is_none() {
            *runtime = Some(Runtime {
                os: os(),
                arch: arch(),
                // version: "1.0.0".to_string(),
            });
        }
        runtime
    }

    #[cfg(test)]
    pub(crate) fn set_os(os: String) {
        let mut runtime = Self::_get();
        runtime.as_mut().unwrap().os = os;
    }

    #[cfg(test)]
    pub(crate) fn set_arch(arch: String) {
        let mut runtime = Self::_get();
        runtime.as_mut().unwrap().arch = arch;
    }

    #[cfg(test)]
    pub(crate) fn reset() {
        let mut runtime = RUNTIME.lock().unwrap();
        *runtime = None;
    }
}

impl UserData for Runtime {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("osType", |_, t| Ok(t.os.clone()));
        fields.add_field_method_get("archType", |_, t| Ok(t.arch.clone()));
        // fields.add_field_method_get("version", |_, t| Ok(t.version.clone()));
    }
}

fn os() -> String {
    match OS {
        "macos" => "darwin".to_string(),
        os => os.to_string(),
    }
}

fn arch() -> String {
    match ARCH {
        "aarch64" => "arm64".to_string(),
        "x86_64" => "x64".to_string(),
        arch => arch.to_string(),
    }
}
