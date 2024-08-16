use std::path::{Path, PathBuf};
use std::{ffi::OsStr, fs, io};

mod dir_and_file_manipulation;
use crate::dir_and_file_manipulation::{create_file, test_create_dir};

fn collect_paths_with_extension(path: &Path, ext: Option<&OsStr>) -> io::Result<Vec<PathBuf>> {
    let mut sgf_paths: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            sgf_paths.extend(collect_paths_with_extension(&path, ext)?);
        } else if path.extension() == ext {
            sgf_paths.push(path);
        }
    }

    Ok(sgf_paths)
}

fn main() -> io::Result<()> {
    test_create_dir();
    create_file();

    let path: &Path = Path::new(".");
    let extension = Some(OsStr::new("sgf"));
    let sgfs = collect_paths_with_extension(path, extension)?;

    for path in sgfs {
        println!("{:?}", path);
    }

    Ok(())
}
