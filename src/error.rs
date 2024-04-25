use mlua::Error as MLuaError;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum VFoxError {
    #[error(transparent)]
    LuaError(#[from] MLuaError),
    #[error("serde_json error")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, VFoxError>;
