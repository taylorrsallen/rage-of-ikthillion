use crate::*;
use serde::*;
use std::env::*;
use std::fs::*;
use std::io::*;

//================================-================================-================================
pub fn try_read_file_to_bytes(
    path: &String,
) -> Option<Vec<u8>> {
    if let Ok(file) = File::open(path) {
        let mut buf_reader = BufReader::new(file);
        let mut bytes: Vec<u8> = vec![];
        
        if let Ok(_) = buf_reader.read_to_end(&mut bytes) {
            if bytes.len() != 0 {
                return Some(bytes);
            }
        }
    }

    None
}

pub fn try_read_file_to_string(
    path: &String,
) -> Option<String> {
    if let Ok(file) = File::open(path) {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        
        if let Ok(_) = buf_reader.read_to_string(&mut contents) {
            if contents.as_str().len() != 0 {
                return Some(contents);
            }
        }
    }

    None
}

//================================-================================-================================
pub fn try_write_file(
    path: &String,
    bytes: &[u8],
) -> bool {
    if let Ok(mut file) = File::create(path) {
        if let Ok(_) = file.write_all(bytes) {
            return true;
        }
    }

    false
}

//================================-================================-================================
pub fn to_ron_string_pretty<T: Serialize>(
    data: &T,
) -> ron::Result<String> {
    ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())
}