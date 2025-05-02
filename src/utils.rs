use std::env;
use std::path::PathBuf;

pub fn get_temp_path() -> PathBuf {
    env::temp_dir().join("temp.sav")
}