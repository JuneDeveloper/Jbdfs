mod file;
mod directory;

use std::fs;
use std::ops::Add;
use directory::FsDirectory;
use file::FsFile;

fn main() {

    //create_new_filesystem("testfs".to_string());

    // Folder:
    // type:name:affectedlines
    // File:
    // type:name:ftype:dataline


    let mut testfile = FsDirectory::new();
    let filename = "testfolder";
    let filepath = "root";
    testfile.filepath = filepath.to_string().add(" -> ").add(filename);
    testfile.filename = filename.to_string();

   // testfile.clone().remove("testfs".to_string()).unwrap();
    //testfile.clone().write("testfs".to_string(), "thisisfile2".to_string()).unwrap();

    //testfile.clone().remove("testfs".to_string()).unwrap();
    //println!("{:?}", testfile.clone().read("testfs".to_string()).unwrap());
}

fn create_new_filesystem(filepath : String) {
    fs::create_dir_all(filepath.clone()).unwrap();
    fs::write(filepath.clone() + "/meta.jbdfsm", "").unwrap();
    fs::write(filepath.clone() + "/data.jbdfs", "").unwrap();

}