use std::env;
use std::process::exit;
use std::path::PathBuf;

mod components;
mod errors;

use components:: {
    handle_input::pre_scrape_setup,
};

use crate::{errors::ErrorVals};

#[derive(Debug, Default)]
struct ScrapedData {
    website: String,
    title: String,
    series_id: String,
    input_url: String,
    _current_chap: i32,
    given_chap: i32,
    pub manga_path: PathBuf,
}

pub const MANGA_LIB_PATH: &str = "/home/fred/Documents/Manga-Lib";

#[tokio::main]
async fn main() -> Result<(),  errors::ErrorVals>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(errors::ErrorVals::NoURL);
    }

    let mut data = ScrapedData::default();
    data.input_url = String::from(&args[1]);

    if args.len() > 2 && !&args[2].is_empty() {
        match args[2].parse::<i32>() {
            Ok(chapter) => data.given_chap = chapter,
            Err(_) => { eprintln!("{} is not a chapter number", args[2]); exit(-1);}
        }
    }
    pre_scrape_setup(&mut data).await?;
    Ok(())
}
