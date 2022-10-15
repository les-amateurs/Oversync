use std::fs;
use std::path::{Path, PathBuf};

struct DatabaseMetadata{

}

impl DatabaseMetadata {
    
}

struct Database {
    pub path: PathBuf,
}

impl Database {
    pub fn new(path_str: String) -> Database{
        return Database {
            path: Path::new(&path_str).to_owned()
        }
    }
}