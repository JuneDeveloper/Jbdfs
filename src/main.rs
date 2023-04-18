mod meta_map;
mod operation;

use std::{env, fs};
use std::fs::{create_dir, File, OpenOptions, remove_dir_all};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::ops::{Add, Index};
use std::path::Path;
use std::process::exit;
use crate::meta_map::MetaMap;
use crate::operation::fsop;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "cfs" {
        make_fs_structure(args[2].as_str());
    } else if args[1] == "rfs" {
        remove_dir_all(args[2].as_str()).unwrap();
    } else {
        let filepath: String = args[3].to_string();
        println!("{:?}", args);

        // Read in data + some checks to make sure the args are valid
        let mut meta = MetaMap::new();
        let mut oper = fsop::new();
        &oper.set_path(filepath.clone());
        if args[2].len() as i8 == 2 {
            oper.operation_type = args[2].chars().nth(0).unwrap().to_digit(10).unwrap() as i8;
            oper.operation_perf = args[2].chars().nth(1).unwrap().to_digit(10).unwrap() as i8;
        } else { exit(1) }
        if oper.is_dir() && (1..3).contains(&(oper.operation_perf as i32)) { exit(1); }
        read_meta(args[1].as_str(), &mut meta).unwrap();

        // Check the file exists, or doesn't, if it's trying to create one
        if !meta.structure.contains_key(&oper.path) {
            // Can't read/write to file/folder if it doesn't exist
            if oper.operation_perf != 4 { exit(10) }
            // Create a file/folder
            else {
                let mut it : i128 = 0;
                let split_path = oper.path.split(" -> ");
                let mut rebuilt_path : String = "".to_string();
                for f in split_path.clone() {
                    if f != "root" { rebuilt_path += " -> "; }
                    rebuilt_path += f;

                    if it != split_path.clone().count() as i128 - 1 && f != "root" && !meta.structure.contains_key(&rebuilt_path) { exit(10) }
                    else if f != "root" && it == split_path.clone().count() as i128 - 1 {
                        if oper.is_dir() { add_dir(args[1].clone(), oper.path.clone()); }
                        else { add_file(args[1].clone(), oper.path.clone(), args[4].clone()); }
                    }
                    it += 1
                }

            }
        } else {
            // Can't create an existing folder
            if oper.operation_perf == 4 { exit(10) }
            else {
                // Read all
                if oper.operation_perf == 0 {
                    if oper.is_file() {
                        let bytes = read_file(args[1].clone(), oper.path.clone());
                        //println!("{:?}", bytes)
                    } else {
                        let mut files = Vec::new();
                        for (path, id) in meta.structure {
                            if path.starts_with(&oper.path) && path.split(" -> ").count() == oper.path.clone().split(" -> ").count() + 1 {
                                files.insert(files.len(), path);
                            }
                        }
                        println!("{:?}", files)
                    }
                }
                // Get file type
                else if oper.operation_perf == 2 {
                    if oper.is_file() {
                        let bytes = get_file_type(args[1].clone(), oper.path.clone());
                       // println!("{:?}", bytes);
                    } else {
                        exit(1);
                    }
                }
                // Write to file
                else if oper.operation_perf == 3 {
                    if oper.is_file() {
                        write_file(args[1].clone(), oper.path.clone(), args[4].clone());
                    } else {
                        exit(1);
                    }
                }
                // Remove file/folder
                else if oper.operation_perf == 5 {
                    if oper.is_file() {
                        remove_file(args[1].clone(), oper.path.clone());
                    } else {
                        remove_dir(args[1].clone(), oper.path.clone());
                    }
                }
                else {
                    exit(1)
                }
            }
        }

    }



}

// Creates a new folder + metadata file + actual filesystem file
fn make_fs_structure(name : &str) {
    create_dir(name).unwrap();
    fs::write(name.to_owned() + "/meta.mjbdfs", "").unwrap();
    fs::write(name.to_owned() + "/fs.jbdfs", "").unwrap();
}

// Write to file
fn write_file(fs : String, path : String, data : String) {
    let file = File::open(fs.clone() + "/fs.jbdfs").unwrap();
    let reader = BufReader::new(file);
    let mut new_data = "".to_string();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with(&path.clone().add(" : ")) {
            let mut file_header = "".to_string();
            let mut count = 0;
            for c in line.chars() {
                if c.to_string() == ":" {
                    count += 1
                }
                file_header += &c.to_string();
                if count == 2 {
                    new_data += (&file_header.clone().add(data.as_str()).add("\r")).as_str();
                    break
                }
            }
        } else {
            new_data += line.add("\r").as_str();
        }
    }
    let mut file = OpenOptions::new().write(true).append(false).open(fs.clone() + "/fs.jbdfs").unwrap();
    writeln!(file, "{}", new_data).unwrap();
}

// Read a file
fn read_file(fs : String, path : String) -> Box<[u8]> {
    let file = File::open(fs.clone() + "/fs.jbdfs").unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with(&path.clone().add(" : ")) {
            let mut file_header = "".to_string();
            let mut count = 0;
            for c in line.chars() {
                if c.to_string() == ":" {
                    count += 1
                }
                file_header += &c.to_string();
                if count == 2 {
                    println!("{}", line.strip_prefix(&file_header).unwrap());
                    return Box::from(line.strip_prefix(&file_header).unwrap().as_bytes());
                }
            }
        }
    }
    return Box::from([0]);
}
// Get file type
fn get_file_type(fs : String, path : String) -> Box<[u8]> {
    let file = File::open(fs.clone() + "/fs.jbdfs").unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with(&path.clone().add(" : ")) {
            let mut filetype = "".to_string();
            let mut count = 0;
            for c in line.chars() {
                if c.to_string() == ":" {
                    count += 1
                }
                else if c.to_string() != " " && count > 0 {
                    filetype += &c.to_string();
                }

                if count == 2 {
                    println!("{}", filetype);
                    return Box::from(filetype.as_bytes());
                }
            }
        }
    }
    return Box::from([0]);
}

// Add/Remove a file
fn add_file(fs : String, path : String, filetype : String) {
    let mut file = OpenOptions::new().write(true).append(true).open(fs.clone() + "/meta.mjbdfs").unwrap();
    writeln!(file, "{}", path).unwrap();
    let mut file = OpenOptions::new().write(true).append(true).open(fs.clone() + "/fs.jbdfs").unwrap();
    writeln!(file, "{}", path + " : " + filetype.as_str() + " :").unwrap();
}
fn remove_file(fs : String, path : String) {
    let file = File::open(fs.clone() + "/fs.jbdfs").unwrap();
    let reader = BufReader::new(file);
    let mut data = "".to_string();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if !line.starts_with(&path.clone().add(" :")) {
            data += line.add("\r").as_str();
        }
    }
    fs::write(fs.clone() + "/fs.jbdfs", data).unwrap();

    let file = File::open(fs.clone() + "/meta.mjbdfs").unwrap();
    let reader = BufReader::new(file);
    let mut data = "".to_string();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line != path.clone() {
            data += line.add("\r").as_str();
        }
    }
    fs::write(fs.clone() + "/meta.mjbdfs", data).unwrap();
}

// Add/Remove a directory
fn add_dir(fs : String, path : String) {
    let mut file = OpenOptions::new().write(true).append(true).open(fs + "/meta.mjbdfs").unwrap();
    writeln!(file, "{}", path).unwrap();
}
fn remove_dir(fs : String, path : String) {
    let file = File::open(fs.clone() + "/meta.mjbdfs").unwrap();
    let reader = BufReader::new(file);
    let mut data = "".to_string();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line != path.clone() {
            data += line.add("\n").as_str();
        }
    }
    fs::write(fs.clone() + "/meta.mjbdfs", data).unwrap();
}
// Read filesystem data
fn read_meta(fs : &str, m : &mut MetaMap) -> Result<(), Error> {
    let f = &fs.to_string().add("/meta.mjbdfs").clone();
    let file = File::open(&Path::new(f))?;
    let file = BufReader::new(file);
    for (num, line) in file.lines().enumerate() {
        m.structure.insert(line?, num as i128);
    }
    Ok(())
}


