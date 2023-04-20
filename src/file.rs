use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};
use std::ops::Add;
use std::path::Path;

pub struct FsFile {
    pub filepath : String,
    pub filename: String,
    pub filetype : String,
    pub id : i128
}
impl FsFile {
    /// Creates a new FsFile struct. Not to be confused with FsFile.create()
    pub fn new() -> FsFile {
        return FsFile { filename: "".to_string(), filetype: "".to_string(), filepath : "".to_string(), id: 0 };
    }

    /// Removes a file. The data will still be there, though inaccessible
    /// Data that doesn't get removed can be purged with a separate call but separating it this way is a lot faster. (Especially with high file counts)
    pub fn remove(self, fs : String) -> std::io::Result<()> {
        let reader = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines();
        let mut dir = "root".to_string();
        // Search for file
        let mut data = "".to_string();
        for line in reader {
            let line : String = line.unwrap().to_string();
            let file_type : i128 = line.clone().split(":").nth(0).unwrap().parse::<i128>().unwrap();
            let file_name : String = line.clone().split(":").nth(1).unwrap().to_string();
            if file_type == 0 && self.filepath.starts_with(&dir.clone().add(" -> ").add(file_name.as_str())) {
                dir = dir.add(" -> ").add(file_name.as_str());
            }
            if !(file_type == 1 && self.filepath == dir.clone().add(" -> ").add(file_name.as_str())) {
                data += line.add("\n").as_str();
            }
        }

        // Gone, reduced to atoms.
        File::create(fs.clone() + "/meta.jbdfsm")?.write_all(data.as_bytes())

    }

    /// Creates a new, blank file, and adds it to meta.jbdfsm and data.jbdfs
    /// If a directory/file with the same name exists, or a parent directory doesn't exist, the directory/file won't be created
    pub fn create(mut self, fs : String) -> std::io::Result<()> {
        let parent_path : String = self.filepath.clone().replace(&" -> ".to_string().add(self.filename.as_str()), "");
        let line_count : usize = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines().count();
        self.id = BufReader::new(File::open(fs.clone() + "/data.jbdfs")?).lines().count() as i128 + 1;

        // Can just write directly with no lines in the file
        let mut data = "".to_string();
        if line_count == 0 {
            if parent_path == "root" {
                fs::write(Path::new(&(fs.clone() + "/meta.jbdfsm")), "1:".to_owned() + &self.filename + ":text:" + self.id.to_string().as_str()).unwrap();
            }
        } else {
            let mut current_directory : String = "root".to_string();
            let path_arr = self.filepath.split(" -> ");
            let path_arr = path_arr.clone();
            let mut path_index : usize = 1;
            let mut ignore_lines : i128 = 0;
            let mut lines_in_directory : i128 = line_count.clone() as i128;
            let mut parent_line : String = "".to_string();
            // Search to make sure the folder definitely exists and the file doesn't already exist
            for line in BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines() {
                let mut search_file_name: String = (path_arr.clone().nth(path_index).unwrap().to_string()).to_string();
                let line = line.unwrap().to_string();
                let file_meta = line.split(":");
                let file_type : i128 = file_meta.clone().nth(0).unwrap().parse::<i128>().unwrap();
                let file_name : String = file_meta.clone().nth(1).unwrap().to_string();
                let file_directory : String = current_directory.clone().add(" -> ").add(file_name.as_str());
                let mut exists : bool = false;
                if current_directory == parent_path && !exists && file_name == self.filename {
                    exists = true;
                }

                if file_directory == parent_path && current_directory != parent_path {
                    parent_line = (&line.clone().to_owned()).to_string();
                }
                if ignore_lines < 1 {
                    if file_type == 0 && parent_path.starts_with(&file_directory) {
                        current_directory = file_directory;
                        search_file_name = file_name.clone();
                        lines_in_directory = file_meta.clone().nth(2).unwrap().to_string().parse::<i128>().unwrap() + 1;
                        path_index += 1

                    }
                    else if file_type == 0 && file_name != search_file_name {
                        ignore_lines = file_meta.clone().nth(2).unwrap().to_string().parse::<i128>().unwrap();
                    }
                    lines_in_directory -= 1;
                } else {
                    ignore_lines -= 1;
                    if file_type == 0 {
                        ignore_lines += file_meta.clone().nth(2).unwrap().to_string().parse::<i128>().unwrap();
                    }
                }
                // Add the new data + Increase the folder subfile count
                data += line.add("\n").as_str();
                if lines_in_directory < 1 && current_directory == parent_path && !exists {
                    if parent_path != "root" {
                        let p_line_meta = parent_line.clone().split(":").nth(2).unwrap().parse::<i128>().unwrap();
                        data = data.replace(&parent_line, parent_line.clone().replace(&":".to_owned().add(p_line_meta.to_string().as_str()), ":".to_owned().add((p_line_meta + 1).to_string().as_str()).as_str()).as_str());
                    }
                    data += ("1:".to_owned() + &self.filename + ":text:" + self.id.to_string().add("\n").as_str()).as_str();
                    exists = true;
                    let mut file = OpenOptions::new().write(true).append(true).open(fs.clone() + "/data.jbdfs").unwrap();
                    writeln!(file, "\n").unwrap();
                }
            }
        }
        let mut file = OpenOptions::new().write(true).append(false).open(fs.clone() + "/meta.jbdfsm").unwrap();
        write!(file, "{}", data).unwrap();
        Ok(())
    }

    /// Writes to a file. All data will be overwritten, if the file isn't valid, it won't be written to.
    /// You cannot write to a directory, only files. All data is lost, there's not any kind of backup, so make sure you write to the correct file.
    pub fn write(mut self, fs : String, dat : String) -> std::io::Result<()> {
        let reader = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines();
        let mut dir = "root".to_string();
        // Search for ID
        for line in reader {
            let line : String = line.unwrap().to_string();
            let file_type : i128 = line.clone().split(":").nth(0).unwrap().parse::<i128>().unwrap();
            let file_name : String = line.clone().split(":").nth(1).unwrap().to_string();
            if file_type == 0 && self.filepath.starts_with(&dir.clone().add(" -> ").add(file_name.as_str())) {
                dir = dir.add(" -> ").add(file_name.as_str());
            }
            else if file_type == 1 && self.filepath == dir.clone().add(" -> ").add(file_name.as_str()) {
                self.id = line.clone().split(":").nth(3).unwrap().parse::<i128>().unwrap() - 1;
                break
            }
        }

        // Write in the new data
        let reader = BufReader::new(File::open(fs.clone() + "/data.jbdfs")?);
        let mut data : String = "".to_string();
        for (n, l) in reader.lines().enumerate() {
            if n as i128 == self.id {
                data += dat.as_str();
            }
            else {
                data += (l.unwrap().as_str().to_string() + "\n").as_str();
            }
        }
        let mut file = OpenOptions::new().write(true).append(false).open(fs.clone() + "/data.jbdfs").unwrap();
        write!(file, "{}", data).unwrap();
        Ok(())
    }

    /// Reads a file. Data will not be touched. Returns either a byte array, or a string.
    /// You can't read a directory, however you can get a list of all subfiles/subdirectories. This returns the raw bytes, but can also return the string value.
    pub fn read(mut self, fs : String) -> Result<String, Error> {
        let reader = BufReader::new(File::open(fs.clone() + "/meta.jbdfsm")?).lines();
        let mut dir = "root".to_string();
        // Search for ID
        for line in reader {
            let line : String = line.unwrap().to_string();
            let file_type : i128 = line.clone().split(":").nth(0).unwrap().parse::<i128>().unwrap();
            let file_name : String = line.clone().split(":").nth(1).unwrap().to_string();
            if file_type == 0 && self.filepath.starts_with(&dir.clone().add(" -> ").add(file_name.as_str())) {
                dir = dir.add(" -> ").add(file_name.as_str());
            }
            else if file_type == 1 && self.filepath == dir.clone().add(" -> ").add(file_name.as_str()) {
                self.id = line.clone().split(":").nth(3).unwrap().parse::<i128>().unwrap() - 1;
                break
            }
        }
        // Read the data
        let data = BufReader::new(File::open(fs.clone() + "/data.jbdfs")?).lines().nth(self.id as usize).unwrap().unwrap();
        Ok(data)




    }
}
impl Clone for FsFile {
    fn clone(&self) -> FsFile {
        return FsFile {id : self.id.clone(), filepath : self.filepath.clone(), filename : self.filename.clone(), filetype : self.filetype.clone() }
    }
}