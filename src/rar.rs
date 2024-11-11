use std::path::PathBuf;

pub(crate) fn unrar(path: &PathBuf) -> std::io::Result<()> {
    let mut cmd = std::process::Command::new("unrar")
        .arg("x")
        .arg("-o-")
        .arg(path)
        .arg(
            path.canonicalize()?
                .parent()
                .expect("since we have a file here, it should have a parent"),
        )
        .spawn()?;

    cmd.wait()?;
    Ok(())
}
