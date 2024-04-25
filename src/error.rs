use mlua::Error as MLuaError;
use thiserror::Error as TError;

#[derive(TError, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("mlua error")]
    LuaError(#[from] MLuaError),
    #[error("serde_json error")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
