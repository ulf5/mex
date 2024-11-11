use std::path::PathBuf;

use crate::{gzip, rar, tar, zip};

pub(crate) enum Archive {
    Rar(PathBuf),
    Zip(PathBuf),
    Tar(PathBuf),
    Gzip(PathBuf),
}

impl Archive {
    pub(crate) fn from_path(path: PathBuf) -> Option<Self> {
        let ext = path.extension()?.to_str()?;
        match ext {
            "rar" => Some(Archive::Rar(path)),
            "zip" => Some(Archive::Zip(path)),
            "gz" => match is_tar(&path) {
                true => Some(Archive::Tar(path)),
                false => Some(Archive::Gzip(path)),
            },
            "bz2" | "xz" | "tbz2" | "tgz" | "txz" => match is_tar(&path) {
                true => Some(Archive::Tar(path)),
                false => None,
            },
            _ => None,
        }
    }

    pub(crate) fn extract(&self) -> std::io::Result<()> {
        match self {
            Archive::Rar(path) => rar::unrar(path)?,
            Archive::Zip(path) => zip::unzip(path)?,
            Archive::Tar(path) => tar::untar(path)?,
            Archive::Gzip(path) => gzip::gunzip(path)?,
        };
        Ok(())
    }
}

fn is_tar(path: &PathBuf) -> bool {
    path.with_extension("")
        .extension()
        .map(|x| x.to_str())
        .flatten()
        == Some("tar")
}
