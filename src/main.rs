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

  //  let mut testdir = FsDirectory::new();
 //  let filename = "folder2";
  //  let filepath = "root";

 //   testdir.filepath = filepath.to_string().add(" -> ").add(filename);
   // testdir.filename = filename.to_string();
    let mut testdir1 = FsDirectory::new();
    let filename = "testfolder";
    let filepath = "root";
    testdir1.filepath = filepath.to_string().add(" -> ").add(filename);
    testdir1.filename = filename.to_string();

    let mut testdir2 = FsDirectory::new();
    let filename = "subtestfolder";
    let filepath = "root -> testfolder";
    testdir2.filepath = filepath.to_string().add(" -> ").add(filename);
    testdir2.filename = filename.to_string();

    let mut testfile = FsFile::new();
    let filename = "testfile3";
    let filepath = "root -> testfolder -> subtestfolder";
    testfile.filepath = filepath.to_string().add(" -> ").add(filename);
    testfile.filename = filename.to_string();


    testdir1.create("testfs".to_string()).unwrap();
    testdir2.create("testfs".to_string()).unwrap();
    testfile.create("testfs".to_string()).unwrap();

}

fn create_new_filesystem(filepath : String) {
    fs::create_dir_all(filepath.clone()).unwrap();
    fs::write(filepath.clone() + "/meta.jbdfsm", "").unwrap();
    fs::write(filepath.clone() + "/data.jbdfs", "").unwrap();

}