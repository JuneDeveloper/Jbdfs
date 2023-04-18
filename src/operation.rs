use std::ops::Add;

pub struct fsop {
    pub operation_type : i8,
    pub operation_perf : i8,
    pub path : String,
}

impl fsop {
    pub fn set_path(&mut self, f : String) {
        let mut filepath = f.clone();
        filepath = filepath.replace(" ", "");
        filepath = filepath.replace("->", " -> ");
        //filepath = filepath.replace("  ", " ");

        if filepath.starts_with(" ") { filepath.remove(0); }
        if filepath.ends_with(" ") { filepath.remove(filepath.len()-1); }

        if !filepath.starts_with("root ->") {
            filepath = "root -> ".to_string().add(&filepath);
        }

        self.path = filepath;
    }

    pub fn new() -> fsop {
        return fsop {operation_type : -1, operation_perf : -1, path : "".to_string() };
    }

    pub fn is_dir(&self) -> bool {
        return self.operation_type == 0;
    }
    pub fn is_file(&self) -> bool {
        return self.operation_type == 1;
    }
}