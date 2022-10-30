use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DatabaseMetadata {
    name: String,
    version: u32,
    open: bool,
    collections: Vec<String>,
}

impl Default for DatabaseMetadata {
    fn default() -> DatabaseMetadata {
        return DatabaseMetadata {
            name: "default database".to_string(),
            version: 1,
            open: true,
            collections: vec![], // empty
        };
    }
}

pub struct Database {
    pub path: PathBuf,
    meta: DatabaseMetadata,
}

impl Database {
    pub fn new(path_str: String) -> Database {
        Database {
            path: Path::new(&path_str).to_owned(),
            meta: DatabaseMetadata::default(),
        }
    }

    // todo: store this somewhere so we don't keep recreating when we need it
    // very microoptimization
    pub fn get_meta_path(&self) -> PathBuf {
        self.path.join("meta.json")
    }

    pub fn create_if_nonexist(&self) -> std::io::Result<()> {
        // println!("creating {:?}",self.path.as_path().to_str());
        fs::create_dir_all(self.path.as_path())?;
        Ok(())
    }

    fn create_directory(&self, name: &str) -> std::io::Result<()> {
        let full_path = self.path.clone().join(name);
        fs::create_dir(full_path)?;
        Ok(())
    }

    pub fn ensure_collection(&mut self, name: &str) -> std::io::Result<()> {
        let owned_name = name.to_owned();
        if !self.meta.collections.contains(&owned_name) {
            self.create_directory(&owned_name)?;
            self.meta.collections.push(owned_name);
        }

        // uncomment if you screw up and there are dups now
        // self.meta.collections.dedup();

        Ok(())
    }

    // TODO: rename meta_contents to a better name
    // TODO: passthrough errors?

    pub fn save_meta(&self) -> std::io::Result<()> {
        let mut file = File::create(self.get_meta_path().as_path()).expect("Meta file open fail. ");
        let meta_contents =
            serde_json::to_string(&self.meta).expect("Serializing meta state failed. ");
        file.write_all(meta_contents.as_bytes())
            .expect("Could not write to metadata file. ");
        Ok(())
    }

    pub fn load_meta(&mut self) -> std::io::Result<()> {
        let meta_contents = fs::read_to_string(self.get_meta_path().as_os_str())?;
        self.meta = serde_json::from_str(&meta_contents)?;
        Ok(())
    }

    fn get_filename(&self, key: &str) -> String {
        format!("{}{}", key, ".json")
    }

    fn get_path_for_key(&self, collection: &str, key: &str) -> PathBuf {
        self.path
            .clone()
            .join(collection)
            .join(self.get_filename(key))
    }

    pub fn put(&self, collection: &str, key: &str, value: &impl Serialize) -> std::io::Result<()> {
        let path = self.get_path_for_key(collection, key);
        let val_str = serde_json::to_string(value);

        return Result::Ok(());
    }

    // picked this name cause serenity used it
    // if someone has a better name like init
    // then sure I might change it

    pub fn start(&mut self) {
        // let manifest_path = self.get_meta_path();
        // TODO: NOT JUST PANIC ON FAILS LOL
        self.create_if_nonexist()
            .expect("Database directory creation fail. ");
        match self.load_meta() {
            Err(_err) => {
                self.save_meta().expect("Initial save of metadata failed. ");
                self.load_meta()
                    .expect("Initial reload of metadata failed. ");
            }
            Ok(_) => return,
        }
    }
}
