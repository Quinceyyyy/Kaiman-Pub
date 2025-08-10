use reqwest::{get};
use std::fs::{File, create_dir};
use std::io::Write;
use std::path::PathBuf;
use rand::Rng;
use tokio::time::{sleep, Duration};


use crate::{ScrapedData, errors::ErrorVals};



pub fn setup_domaine_api(data: &ScrapedData) -> String
{
    let mut api_string =  String::new();

    match data.website.as_str() {
        "weebcentral.com" => { api_string = format!("https://weebcentral.com/series/{}/full-chapter-list", data.series_id);}
        "mangadex.org" => { api_string = format!("https://api.mangadex.org/chapter?manga={}&translatedLanguage[]=en&order[chapter]=asc&limit=100", data.series_id);}
        &_ => {}

    }
    return api_string;
}

pub fn create_api_call(chapter_link: &str, data: &ScrapedData) -> Result<String, ErrorVals>
{
    let chap_id = chapter_link.rsplit('/').next().ok_or(ErrorVals::InvalidURL)?;

    if data.website == "mangadex.org" {
        let _mangadex_api= format!("https://mangadex.org/{}/images?is_prev=False&current_page=1&reading_style=long_strip", chap_id);
    }

    let weebcentral_api = format!("https://weebcentral.com/chapters/{}/images?is_prev=False&current_page=1&reading_style=long_strip", chap_id);

    Ok(weebcentral_api)
}

pub async fn image_downloader(page_link: &str, idx: usize, chap_dir: &PathBuf) -> Result<(), ErrorVals>
{
    let img_request= get(page_link).await.map_err(ErrorVals::HttpError)?;
    let img_data = img_request.bytes().await.map_err(ErrorVals::HttpError)?;

    let mut dl_path = chap_dir.clone();
    dl_path.push(format!("page_{:03}", idx));

    let mut img_file = File::create(&dl_path).map_err(ErrorVals::IoError)?;
    img_file.write_all(&img_data).map_err(ErrorVals::IoError)?;

    Ok(())
}

pub fn write_chap_dir(data: &ScrapedData, chap_num: usize) -> Result<Option<PathBuf>, ErrorVals>
{
    let nbr = chap_num + 1;
    let chapter_path = format!("chapter_{}", nbr);
    let temp_path = data.manga_path.join(&chapter_path);

    if !temp_path.exists() {
        create_dir(&temp_path).map_err(ErrorVals::IoError)?;
        println!("chapter_{} dir was created", nbr);
        return Ok(Some(temp_path));
    }
    Ok(None)
}

pub async fn random_delay() {
    let delay_ms = rand::rng().random_range(2000..5000);
    sleep(Duration::from_millis(delay_ms)).await;
}