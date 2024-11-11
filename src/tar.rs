use std::path::PathBuf;


pub(crate) fn untar(path: &PathBuf) -> std::io::Result<()> {
    let mut cmd = std::process::Command::new("tar")
        .arg("zxvf")
        .arg(path)
        .spawn()?;

    cmd.wait()?;

    Ok(())
}
