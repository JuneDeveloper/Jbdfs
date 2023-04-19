mod file;
mod directory;

use std::fs;
use std::ops::Add;
use directory::FsDirectory;

fn main() {

    //create_new_filesystem("testfs".to_string());

    // Folder:
    // type:name:affectedlines

    let mut testdir = FsDirectory::new();
    let filename = "folder2";
    let filepath = "root";

    testdir.filepath = filepath.to_string().add(" -> ").add(filename);
    testdir.filename = filename.to_string();


    println!("{} <-> {}", testdir.filename, testdir.filepath);
    testdir.get_listing("testfs".to_string()).unwrap();
}

fn create_new_filesystem(filepath : String) {
    fs::create_dir_all(filepath.clone()).unwrap();
    fs::write(filepath.clone() + "/meta.jbdfsm", "").unwrap();
    fs::write(filepath.clone() + "/data.jbdfs", "").unwrap();

}