use reqwest::{Client};
use tokio::time::{sleep, Duration};
use std::{fs, path::PathBuf};

use crate::{
    utils::{
        api_helper::{setup_domaine_api},
        image_helpers::{image_downloader},
        chapter_dir_helpers::{write_chap_dir, complete_chapter, check_completed_marker},
    },
    errors::ErrorVals, 
    ScrapedData,
};

use crate::components::mangadex::json_structs::{
    ChapterListResp,
    HomeResp,
    MangadexResp,
    CoverResp,
};


async fn download_mangadex_cover(data: &ScrapedData, mangadex_client: &Client) -> Result<(), ErrorVals>
{
    let mangadex_url = format!("https://api.mangadex.org/manga/{}?includes[]=cover_art", data.series_id);
    let mangadex_resp = mangadex_client
        .get(&mangadex_url)
        .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
        .send()
        .await?
        .json::<MangadexResp>()
        .await?;

    let mut temp_cover_id = None;
    for relationship in mangadex_resp.data.relationships.iter() {
        if relationship.rel_type == "cover_art" {
            temp_cover_id = Some(&relationship.id);
            break;
        }
    }
    let cover_id = match temp_cover_id {
        Some(id) => id,
        None => {
            println!("Couldnt find cover art for: {}", data.title);
            return Err(ErrorVals::CoverNotFound)
        }
    };

    let cover_url = format!("https://api.mangadex.org/cover/{}", cover_id);
    let cover_resp = mangadex_client
        .get(&cover_url)
        .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
        .send()
        .await?
        .json::<CoverResp>()
        .await?;

    let cover_link = format!("https://uploads.mangadex.org/covers/{}/{}", data.series_id, cover_resp.data.attributes.filename);
    let ext = cover_resp.data.attributes.filename
        .split('.')
        .last()
        .unwrap_or("jpg");

    let cover_name = format!("{}_cover.{}", data.title, ext);
    let cover_path = data.manga_path.join(cover_name);

    if cover_path.exists() {
        println!("Cover for: {} has already been dowloaded", data.title);
        return Ok(());
    }
    let cover_request = mangadex_client
        .get(&cover_link)
        .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
        .send()
        .await?;
    let cover_bytes = cover_request.bytes().await?;

    fs::write(&cover_path, &cover_bytes)?;
    println!("Cover for '{}' has been downloaded to: {} ", data.title, data.title);

    Ok(())
}

async fn scrape_images(home_rep: HomeResp, data: &ScrapedData, chap_num: usize, client: &Client) -> Result<(), ErrorVals>
{
    let chap_dir = match write_chap_dir(data, chap_num)? {
        Some(chap_dir) => chap_dir,
        None => {
            let chap_dir: PathBuf = data.manga_path.join(format!("chapter_{}", chap_num + 1));

            if check_completed_marker(&chap_dir) {
                println!("Chapter {} was already completed, skipping to the next chapter !", chap_num + 1);
                return Ok(());
            }
            println!("Chapter {} was incomplete, resuming download of the chapter", chap_num + 1);
            chap_dir
        }
    };

    for (idx, page) in home_rep.chapter.data.iter().enumerate() {
        let img_url = format!("{}/data/{}/{}", home_rep.base_url, home_rep.chapter.hash, page);
        println!("[{}/{}] Downloading: {}", idx + 1, home_rep.chapter.data.len(), img_url);
        image_downloader(&img_url, idx, &chap_dir, data, client).await?;
    }
    complete_chapter(&chap_dir, data, chap_num)?;
    Ok(())
}


pub async fn scrape_mangadex(data: &ScrapedData) -> Result<(), ErrorVals>
{
    let client: Client = Client::new();
    let api_url = setup_domaine_api(data);

    download_mangadex_cover(data, &client).await?;

    let chapters_rep = client
        .get(&api_url)
        .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
        .send()
        .await?
        .json::<ChapterListResp>()
        .await?;

    println!("current chapters: {}", chapters_rep.data.len());

    for (idx, res) in chapters_rep.data.iter().enumerate() {
        let chap_id = &res.id;

        let home_url = format!("https://api.mangadex.org/at-home/server/{}", chap_id);
        let home_rep = client
            .get(&home_url)
            .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
            .send()
            .await?
            .json::<HomeResp>()
            .await?;
    
        scrape_images(home_rep, data, idx, &client).await?;
        sleep(Duration::from_millis(1000)).await;
    }

    println!("{} has been scraped", data.title);
    Ok(())
}