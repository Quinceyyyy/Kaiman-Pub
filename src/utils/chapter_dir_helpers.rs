use std::path::PathBuf;
use std::fs::{create_dir, write};

use crate::{ScrapedData, errors::ErrorVals};

pub const COMPLETED_CHAPTER: &str = ".completed";






pub fn write_chap_dir(data: &ScrapedData, chap_num: usize) -> Result<Option<PathBuf>, ErrorVals>
{
    let nbr = chap_num + 1;
    let chapter_path = format!("chapter_{}", nbr);
    let temp_path = data.manga_path.join(&chapter_path);

    if !temp_path.exists() {
        create_dir(&temp_path).map_err(ErrorVals::IoError)?;
        println!("chapter {} dir was created", nbr);
        return Ok(Some(temp_path));
    }
    Ok(None)
}

pub fn complete_chapter(chap_dir: &PathBuf, data: &ScrapedData, chap_num: usize) -> Result<(), ErrorVals>
{
    let path = data.manga_path.join(&chap_dir).join(COMPLETED_CHAPTER);

    write(path, "")?;
    println!("chapter {} has been completed", chap_num + 1);
    Ok(())
}

pub fn check_completed_marker(chap_dir: &PathBuf) -> bool
{
    let completed_chap = chap_dir.join(COMPLETED_CHAPTER);

    if completed_chap.exists() {
        return true;
    }
    return false;
}
