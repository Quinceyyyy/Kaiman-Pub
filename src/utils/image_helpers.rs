use reqwest::{get, Client};
use std::path::{PathBuf};
use std::fs::{write};


use crate::{ScrapedData, errors::ErrorVals};



pub async fn download_cover(data: &ScrapedData) -> Result<(), ErrorVals>
{
    let cover_link = format!("https://temp.compsci88.com/cover/fallback/{}.jpg", data.series_id);
    let cover_name = format!("{}_cover", data.title);
    let cover_path: PathBuf = data.manga_path.join(&cover_name);

    if cover_path.exists() {
        println!("{} is already downloaded in {}/{}", cover_name, data.manga_path.display(), data.title);
        return Ok(());
    }
 
    let cover_img_request= get(cover_link).await.map_err(ErrorVals::HttpError)?;
    let cover_img_data = cover_img_request.bytes().await.map_err(ErrorVals::HttpError)?;

    write(&cover_path, &cover_img_data).map_err(ErrorVals::IoError)?;
    println!("Cover for '{}' has been downloaded to: {} ", data.title, data.title);
    Ok(())
}


pub async fn image_downloader(page_link: &str, idx: usize, chap_dir: &PathBuf, data: &ScrapedData, page_client: &Client) -> Result<(), ErrorVals>
{
    let page_path: PathBuf = chap_dir.join(format!("page_{:03}", idx + 1));

    if page_path.exists() {
        println!("Page {:03} already downloaded skipping to next page", idx + 1);
        return Ok(());
    }

    let img_request = match data.website.as_str() {
        "weebcentral.com" => {
            page_client.get(page_link).send().await.map_err(ErrorVals::HttpError)?
        },
        "mangadex.org" => {
            page_client
                .get(page_link)
                .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
                .send()
                .await?
        },
        _ => return Ok(())
        
    };

    let img_data = img_request.bytes().await.map_err(ErrorVals::HttpError)?;
    write(&page_path, &img_data)?;

    Ok(())
}