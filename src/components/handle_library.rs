use std::fs::{create_dir};
use std::path::PathBuf;

use crate::{ErrorVals, ScrapedData};


pub async fn setup_library(data: &mut ScrapedData) -> Result<(), ErrorVals>
{
    let title_dir = PathBuf::from(&data.user_path).join(&data.title);

    if !title_dir.exists() {
        create_dir(&title_dir).map_err(ErrorVals::IoError)?;
        println!("{} dir has been created in {} !", data.title, &data.user_path);
    }

    data.manga_path = title_dir.clone();
    Ok(())
}
