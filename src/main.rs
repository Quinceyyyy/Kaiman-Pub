use std::env;
use std::process::exit;
use std::path::PathBuf;

mod components;
mod errors;
mod utils;

use components:: {
    handle_input::pre_scrape_setup,
    write_user_path::{ check_input_path, read_save_file },
};

use crate::{errors::ErrorVals};

#[derive(Debug, Default)]
struct ScrapedData {
    website: String,
    title: String,
    series_id: String,
    input_url: String,
    user_path: String,
    _current_chap: u32,
    given_chap: u32,
    pub manga_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(),  errors::ErrorVals>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(errors::ErrorVals::NoURL);
    }

    let mut data = ScrapedData::default();

    if args.len() == 2 {
        data.input_url = args[1].clone();
        data.user_path = read_save_file();
        check_input_path(&data)?;
        pre_scrape_setup(&mut data).await?;
        return Ok(());
    }
    data.user_path = args[1].clone();
    data.input_url = args[2].clone();

    check_input_path(&data)?;
    read_save_file();
    if args.len() > 2 && !&args[3].is_empty() {
        match args[3].parse::<u32>() {
            Ok(chapter) => data.given_chap = chapter,
            Err(_) => { eprintln!("{} is not a chapter number", args[3]); exit(-1);}
        }
    }
    pre_scrape_setup(&mut data).await?;
    Ok(())
}
