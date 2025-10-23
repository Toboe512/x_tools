mod errors;
mod file;
mod metadata;
mod utils;

use crate::errors::log_err;
use std::ffi::OsStr;

use crate::metadata::delete_metadata;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Seek, Write};
use std::path::PathBuf;

fn main() -> Result<(), io::Error> {
    match fs::read_dir(".") {
        Ok(paths) => {
            for path in paths {
                let file_path = path?.path();
                let file_name = file_path
                    .file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");
                if !identify_jpg(&file_path).unwrap_or_else(|e1| {
                    println!("File: {} ERR: {}: ", file_name, e1);
                    false
                }) {
                    continue;
                }

                let tmp_path = del_ext(file_name).to_owned() + "_cl.jpg";

                let tmp_file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&tmp_path)
                    .map_err(|e2| io::Error::new(ErrorKind::Other, e2))?;

                println!("TEMP File create new OK: {}", tmp_path);

                let mut reader = BufReader::new(File::open(file_name)?);

                println!("File read OK: {}", file_name);
                let mut writer = BufWriter::new(tmp_file);

                delete_metadata(&mut reader, &mut writer)
                    .map_err(|e2| io::Error::new(ErrorKind::Other, format!("{:?}", e2)))?;
            }
        }
        Err(e) => panic!("Ошибка чтения директории:{}", e),
    }

    Ok(())
}

fn identify_jpg(path: &PathBuf) -> Result<bool, String> {
    let name = path
        .file_name()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("");

    if !is_jpg(name) {
        return Ok(false);
    }

    let mut source = File::open(&path).map_err(|e| log_err(e))?;
    let mut signature = [0; 8];
    source
        .read_exact(&mut signature)
        .map_err(|e| log_err(e))
        .map_err(|e| log_err(e))?;
    Ok(signature[0] == 0xFF && signature[1] == 0xD8 && signature[2] == 0xFF)
}

fn is_jpg(file_name: &str) -> bool {
    if file_name.is_empty() {
        return false;
    }

    let name = file_name.to_lowercase();
    let index_p = name.find('.').unwrap_or(0);
    let name_len = name.len();

    if name_len < 5 || index_p == 0 {
        return false;
    }

    let ext = &name[index_p + 1..name_len];

    ext == "jpeg" || ext == "jpg"
}

fn del_ext(file_name: &str) -> String {
    let name = file_name.to_lowercase();
    let Some(index_p) = name.find('.') else {
        return name.to_string();
    };
    name[0..index_p].to_string()
}
