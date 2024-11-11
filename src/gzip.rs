
use std::path::PathBuf;


pub(crate) fn gunzip(path: &PathBuf) -> std::io::Result<()> {
    let mut cmd = std::process::Command::new("gunzip")
        .arg(path)
        .spawn()?;

    cmd.wait()?;

    Ok(())
}
