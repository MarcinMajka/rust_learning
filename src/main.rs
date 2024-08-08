use std::path::Path;
use std::{ffi::OsStr, fs, io};

fn print_paths_to_files_with_extension(path: &Path, ext: Option<&OsStr>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            print_paths_to_files_with_extension(&path, ext)?;
        } else if path.extension() == ext {
            println!("Path: {:?}", path);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let path: &Path = Path::new(".");
    let extension = Some(OsStr::new("sgf"));
    print_paths_to_files_with_extension(path, extension)?;

    Ok(())
}
