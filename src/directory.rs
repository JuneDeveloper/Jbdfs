use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::ops::Add;
use std::path::Path;
use std::thread::AccessError;

pub struct FsDirectory {
    pub filename : String,
    pub filepath : String
}
impl FsDirectory {
    /// Creates a new FsDirectory struct. Not to be confused with FsDirectory.create()
    pub fn new() -> FsDirectory {
        return FsDirectory { filename: "".to_string(), filepath : "".to_string()};
    }

    /// Removes a directory.
    /// This will completely remove it - however a directory must have no subdirectories to be deleted.
    pub fn remove(self, fs : String) -> std::io::Result<()> {
        let mut reader = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines();
        let mut dir = "root".to_string();
        // Search for file
        let mut data = "".to_string();
        let mut can_overwrite = true;
        for line in reader {
            let line : String = line.unwrap().to_string();
            let file_type : i128 = line.clone().split(":").nth(0).unwrap().parse::<i128>().unwrap();
            let file_name : String = line.clone().split(":").nth(1).unwrap().to_string();
            if file_type == 0 && self.filepath.starts_with(&dir.clone().add(" -> ").add(file_name.as_str())) {
                dir = dir.add(" -> ").add(file_name.as_str());
            }
            if !(file_type == 0 && self.filepath == dir) {
                data += line.add("\n").as_str();
            }
            else if file_type == 0 && self.filepath == dir {
                let subdirectories : i128 = line.clone().split(":").nth(2).unwrap().parse::<i128>().unwrap();
                if subdirectories != 0 {
                    data += line.add("\n").as_str();
                }
                can_overwrite = false;
                break
            }
        }

        // Gone, reduced to atoms.
        if can_overwrite {
            File::create(fs.clone() + "/meta.jbdfsm")?.write_all(data.as_bytes())
        } else {
            Ok(())
        }

    }

    /// Adds a directory to meta.jbdfsm
    /// If a directory/file with the same name exists, or a parent directory doesn't exist, the directory/file won't be created
    pub fn create(self, fs : String) -> std::io::Result<()> {
        let split_path = self.filepath.split(" -> ").enumerate();
        // Finds the parent folder/directory root
        let mut parent_path : String = "".to_string();
        let folder_count : usize = split_path.clone().count();
        let parent_directory: String = split_path.clone().nth(folder_count - 2).unwrap().1.to_string();
        for (n , folder) in split_path {
            if n == folder_count - 2 {
                parent_path += folder;
                break
            } else {
                parent_path += (folder.to_string() + " -> ").as_str();
            }
        }
        let parent_path : String = parent_path.to_string();
        let line_count : usize = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines().count();
        // If there's nothing in the meta file, it can just write directly to it, since there's not going to be anything important there
        if line_count == 0 {
            // The root directory would be the only valid folder location
            if parent_directory == "root" {
                fs::write(Path::new(&(fs.clone() + "/meta.jbdfsm")), "0:".to_owned() + &self.filename + ":0").unwrap();
            }
        }
        // If there's data in the meta file, it needs to be parsed first!
        else {
            let metadata_lines = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines();
            let mut lines_left_in_directory : i128 = line_count.clone() as i128;
            let mut current_directory : String = "root".to_string();
            let mut current_directory_subfiles : i128 = 0;
            let mut ignore_lines : i128 = 0;
            let mut data : String = "".to_string();
            let mut added_file : bool = false;

            for line in metadata_lines{
                let line : String = line.unwrap();

                if !added_file && ignore_lines == 0 {
                    let file_metadata = line.split(":");
                    let file_type : i128 = file_metadata.clone().nth(0).unwrap().to_string().parse::<i128>().unwrap();
                    // Check if it's a directory and whether it should search its subfiles or not
                    if file_type == 0 {
                        let directory_name : String = file_metadata.clone().nth(1).unwrap().to_string();
                        let directory_path : String = current_directory.clone().add(" -> ").add(directory_name.as_str());
                        let directory_subfile_count: i128 = file_metadata.clone().nth(2).unwrap().to_string().parse::<i128>().unwrap();

                        // Can't create a file if it already exists!
                        if directory_name == self.filename && current_directory == parent_path {
                            added_file = true;
                        }
                        // Found a parent directory!
                        else if parent_path.starts_with(&directory_path) {
                            current_directory_subfiles = directory_subfile_count;
                            lines_left_in_directory = directory_subfile_count + 1;
                            current_directory = directory_path.clone();
                        }
                        // Ignore subfiles if the directory isn't relevant
                        else { ignore_lines = directory_subfile_count; }

                    } else {
                        let file_name : String = file_metadata.clone().nth(1).unwrap().to_string();
                        if file_name == self.filename && current_directory == parent_path {
                            added_file = true;
                        }
                    }
                }
                if ignore_lines > 0 { ignore_lines -= 1 }
                lines_left_in_directory -= 1;

                // Update parent folder + Append new line's data
                data += (line.as_str().to_owned() + "\n").as_str();
                if lines_left_in_directory <= 0 && self.filepath == current_directory.clone().add(" -> ").add(&self.filename) && !added_file {
                    data += ("0:".to_owned() + &self.filename + ":0\n").as_str();
                    if parent_directory != "root" {
                        let folder_name = current_directory.split(" -> ");
                        let folder_name = folder_name.clone().nth(folder_name.count() - 1).unwrap();

                        let original_line : String = "0:".to_string().add(folder_name).add(":").add(current_directory_subfiles.to_string().as_str());
                        let replaced_line : String = "0:".to_string().add(folder_name).add(":").add((current_directory_subfiles + 1).to_string().as_str());
                        data = data.replace(&original_line, replaced_line.as_str());
                    }
                    added_file = true
                }
            }
            println!("\n{}", data);
            fs::write(Path::new(&(fs.clone() + "/meta.jbdfsm")), data).unwrap();
        }

        Ok(())
    }

    /// Returns an array with all the sub files/directories of a given directory
    /// Returns the names, that's all, subfolders aren't included here
    pub fn get_listing(self, fs : String) -> std::io::Result<Vec<String>>{
        // Finds the parent folder/directory root
        let mut data : Vec<String> = Vec::new();
        let line_count : usize = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines().count();

        if line_count > 0 {
            let metadata_lines = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines();
            let mut current_directory : String = "root".to_string();
            let mut ignore_lines : i128 = 0;
            let mut folder_files : i128 = 0;
            let mut found : bool = false;

            for line in metadata_lines{
                let line : String = line.unwrap();

                // Metadata
                let file_metadata = line.split(":");
                let file_name : String = file_metadata.clone().nth(1).unwrap().to_string();
                let file_path : String = current_directory.clone().add(" -> ").add(file_name.as_str());
                let file_type : i128 = file_metadata.clone().nth(0).unwrap().to_string().parse::<i128>().unwrap();

                // Add sub directories/files to Vec<String>
                if found {
                    if ignore_lines < 1 {
                        data.insert(data.len(), file_path.clone());
                        if folder_files == 0 { break }
                        folder_files -= 1;
                    } else { ignore_lines -= 1; }
                }
                // Search for target folder
                if file_type == 0 {
                    let directory_subfile_count: i128 = file_metadata.clone().nth(2).unwrap().to_string().parse::<i128>().unwrap();
                    if !found {
                        if self.filepath == file_path {
                            folder_files = directory_subfile_count;
                            current_directory = file_path.clone();
                            found = true;

                        } else if self.filepath.starts_with(&file_path) && !found { current_directory = file_path.clone(); }
                    } else {
                        ignore_lines += directory_subfile_count;
                    }
                }
            }
        }
        Ok(data)
    }
}
impl Clone for FsDirectory {
    fn clone(&self) -> FsDirectory {
        return FsDirectory { filepath : self.filepath.clone(), filename : self.filename.clone() }
    }
}