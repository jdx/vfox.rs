use mlua::Error as MLuaError;
use thiserror::Error;
use xx::XXError;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum VFoxError {
    #[error(transparent)]
    LuaError(#[from] MLuaError),
    #[error("serde_json error")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    XXError(#[from] XXError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
}

pub type Result<T> = std::result::Result<T, VFoxError>;
