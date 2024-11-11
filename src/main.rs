use std::{env, fs::read_dir, path::Path};
use archive::Archive;

mod archive;
mod gzip;
mod rar;
mod tar;
mod zip;

fn main() -> std::io::Result<()> {
    let mut archives = vec![];

    for arg in env::args().skip(1) {
        archives.extend(find_archives(&arg)?);
    }

    let static_archives = archives.leak();
    for chunk in static_archives.chunks(10) {
        let mut handles = vec![];
        for archive in chunk {
            handles.push(std::thread::spawn(move || {
                archive
                    .extract()
                    .map_err(|e| eprintln!("extraction failed: {e:?}"))
                    .ok();
            }));
        }
        for handle in handles {
            handle.join().expect("our threads hopefully dont panic");
        }
    }
    Ok(())
}

fn find_archives(path_str: &str) -> std::io::Result<Vec<Archive>> {
    let path = Path::new(path_str);
    if path.is_file() {
        return match Archive::from_path(path.to_path_buf()) {
            Some(archive) => Ok(vec![archive]),
            None => Ok(vec![]),
        }
    }
    let entries = read_dir(path)?;
    let mut res = vec![];
    for entry in entries {
        let epath = entry?.path();
        if epath.is_file() {
            match Archive::from_path(epath) {
                Some(archive) => res.push(archive),
                None => continue,
            }
        }
    }

    Ok(res)
}
