use std::path::PathBuf;

pub(crate) fn unzip(path: &PathBuf) -> std::io::Result<()> {
    let mut cmd = std::process::Command::new("unzip")
        .arg("-n")
        .arg(path)
        .arg("-d")
        .arg(
            path.canonicalize()?
                .parent()
                .expect("since we have a file here, it should have a parent"),
        )
        .spawn()?;

    cmd.wait()?;
    Ok(())
}
