use std::path::PathBuf;
use std::fs::{ read_to_string, write};


use crate::{ScrapedData, ErrorVals};

pub const SAVE_FILE: &str = "./path_save.txt";


pub fn read_save_file() -> String
{
    if let Ok(save_file) = read_to_string(SAVE_FILE).map_err(ErrorVals::IoError) {
        return save_file;
    }
    return String::new();
}

fn write_to_save(path: &String) -> Result<(), ErrorVals>
{
    write(SAVE_FILE, path).map_err(ErrorVals::IoError)?;
    Ok(())
}

pub fn check_input_path(data: &ScrapedData) -> Result<(), ErrorVals>
{
    let given_path: PathBuf = PathBuf::from(&data.user_path);

    if !given_path.exists() {
        return Err(ErrorVals::InvalidPath);
    }
    write_to_save(&data.user_path)?;
    Ok(())
}
