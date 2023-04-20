mod file;
mod directory;

use std::{env, fs};
use std::fs::remove_dir_all;
use std::ops::Add;
use directory::FsDirectory;
use file::FsFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args[1].starts_with("-") && args.len() >= 4 {
        let action_t : i128 = args[1].chars().nth(0).clone().unwrap().to_string().parse::<i128>().unwrap();
        let action_a : i128 = args[1].chars().nth(1).clone().unwrap().to_string().parse::<i128>().unwrap();
        let path : String = args[2].clone();
        let mut filepath: String = args[3].clone();
        filepath = filepath.replace(" ", "");
        filepath = filepath.replace("->", ">");
        filepath = filepath.replace("<-", ">");
        filepath = filepath.replace("<->", ">");
        filepath = filepath.replace("<", ">");
        filepath = filepath.replace("/", ">");
        filepath = filepath.replace("\\", ">");
        filepath = filepath.replace(">", " -> ");
        if filepath.starts_with("root -> ") {
            filepath = "root -> ".to_string().add(filepath.as_str())
        }
        // Folders
        if action_t == 0 {
            if action_a == 0 {
                let mut f : FsDirectory = FsDirectory::new();
                f.filepath = filepath.clone();
                f.filename = filepath.split(" -> ").last().unwrap().to_string();
                f.create(path).unwrap();
            }
            else if action_a == 1 {
                let mut f : FsDirectory  = FsDirectory::new();
                f.filepath = filepath.clone();
                f.filename = filepath.split(" -> ").last().unwrap().to_string();
                f.remove(path).unwrap();
            }
            else if action_a == 2 {
                let mut f : FsDirectory  = FsDirectory::new();
                f.filepath = filepath.clone();
                f.filename = filepath.split(" -> ").last().unwrap().to_string();
                println!("{:?}", f.get_listing(path).unwrap());
            }
        }
        // Files
        else if action_t == 1 {
            if action_a == 0 {
                let mut f : FsFile = FsFile::new();
                f.filepath = filepath.clone();
                f.filename = filepath.split(" -> ").last().unwrap().to_string();
                f.create(path).unwrap();
            }
            else if action_a == 1 {
                let mut f : FsFile  = FsFile::new();
                f.filepath = filepath.clone();
                f.filename = filepath.split(" -> ").last().unwrap().to_string();
                f.remove(path).unwrap();
            }
            else if action_a == 2 {
                let mut f : FsFile  = FsFile::new();
                f.filepath = filepath.clone();
                f.filename = filepath.split(" -> ").last().unwrap().to_string();
                println!("{}", f.read(path).unwrap());
            }
            else if action_a == 3 {
                if args.len() == 5 {
                    let mut f : FsFile  = FsFile::new();
                    f.filepath = filepath.clone();
                    f.filename = filepath.split(" -> ").last().unwrap().to_string();
                    f.write(path, args[4].clone()).unwrap();
                }
               else {
                   println!("Invalid arguments!");
               }
            }
        }
        else {
            println!("Filetype does not exist!")
        }
    }
    else if args[1] == "-cfs" && args.len() > 2 {
        create_new_filesystem(args[2].clone());
    }
    else if args[1] == "-rfs" && args.len() > 2 {
        remove_dir_all(args[2].clone()).unwrap();
    }
    else {
        println!("Invalid arguments!");
    }
}

fn create_new_filesystem(filepath : String) {
    fs::create_dir_all(filepath.clone()).unwrap();
    fs::write(filepath.clone() + "/meta.jbdfsm", "").unwrap();
    fs::write(filepath.clone() + "/data.jbdfs", "").unwrap();

}