use std::path::Path;
use std::{ffi::OsStr, fs, io};

fn visit_dirs(path: &Path, cb: &dyn Fn(&Path) -> io::Result<()>) -> io::Result<()> {
    cb(path)?;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path, cb)?;
        }
    }
    Ok(())
}

fn print_dir_contents(path: &Path) -> io::Result<()> {
    let mut paths: Vec<_> = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    paths.sort();

    println!("Folder name: {:?}, its contents:", path);
    println!("{:?}\n\n", paths);

    Ok(())
}

fn main() -> io::Result<()> {
    let path: &Path = Path::new("./test_folder");
    visit_dirs(path, &print_dir_contents)?;

    Ok(())
}
