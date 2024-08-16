use std::fs;

pub fn create_file() {
    let path = "./dir/file01.txt";
    let path2 = "./dir/file02.txt";
    let path3 = "./dir/file03.txt";
    let text = "Text inside the file";
    let text2 = "Text inside the file 2";
    let text3 = "Text inside the file 3";
    _ = fs::write(path, text);
    _ = fs::write(path2, text2);
    _ = fs::write(path3, text3);
}

pub fn test_create_dir() {
    let path = "./dir";
    let my_path = std::path::Path::new(path);

    if my_path.exists() {
        println!("Dir already exists. Skipping!");
        return;
    }

    let create_dir_result = fs::create_dir(path);
    if create_dir_result.is_ok() {
        println!("Created a new dir");
    } else {
        println!("DIDN'T create a new dir - {:?}", create_dir_result.err());
    }
}
