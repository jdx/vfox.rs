use std::path::{Path, PathBuf};
use mlua::{ExternalResult, Lua, Table, Value, ExternalError, IntoLuaMulti, MultiValue};
use crate::error::Result;

pub fn mod_archiver(lua: &Lua) -> Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    Ok(loaded.set("archiver", lua.create_table_from(vec![
        ("decompress", lua.create_async_function(decompress)?),
    ])?)?)
}

async fn decompress<'lua>(_lua: &'lua Lua, input: MultiValue<'lua>) -> mlua::Result<()> {
    let paths = input.into_vec();
    let archive: PathBuf = PathBuf::from(paths[0].to_string()?);
    let destination: PathBuf = PathBuf::from(paths[1].to_string()?);
    if archive.ends_with(".zip") {
        unzip(&archive, &destination).into_lua_err()?;
    } else if archive.ends_with(".tar.gz") {
        untar_gz(&archive, &destination).into_lua_err()?;
    } else if archive.ends_with(".tar.xz") {
        untar_xz(&archive, &destination).into_lua_err()?;
    } else if archive.ends_with(".tar.bz2") {
        untar_bz2(&archive, &destination).into_lua_err()?;
    } else {
        unimplemented!("Unsupported archive format {:?}", archive);
    }
    Ok(())
}

fn untar_gz(archive: &Path, destination: &Path) -> Result<()> {
    let file = std::fs::File::open(&archive)?;
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
    archive.unpack(&destination)?;
    Ok(())
}

fn untar_xz(archive: &Path, destination: &Path) -> Result<()> {
    let file = std::fs::File::open(&archive)?;
    let mut archive = tar::Archive::new(xz::read::XzDecoder::new(file));
    archive.unpack(&destination)?;
    Ok(())
}

fn untar_bz2(archive: &Path, destination: &Path) -> Result<()> {
    let file = std::fs::File::open(&archive)?;
    let mut archive = tar::Archive::new(bzip2::read::BzDecoder::new(file));
    archive.unpack(&destination)?;
    Ok(())
}

fn unzip(archive: &Path, destination: &Path) -> Result<()> {
    let file = std::fs::File::open(&archive)?;
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = destination.join(file.name());
        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                std::fs::create_dir_all(p)?;
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_decompress() {
        // todo
    }
}
