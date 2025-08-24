use std::path::PathBuf;

use reqwest::Client;
use scraper::{self, Selector, Html};

use crate::{
    utils::{
        api_helper::{setup_domaine_api, create_api_call},
        image_helpers::{image_downloader, download_cover},
        chapter_dir_helpers::{write_chap_dir, complete_chapter, check_completed_marker},
        misc::{random_delay},
    },
    errors::ErrorVals, 
    ScrapedData,
};


async fn scrape_images(chapter_link: &str, data: &ScrapedData, chap_num: usize, client: &Client) -> Result<(), ErrorVals>
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

    let api_call = create_api_call(chapter_link)?;

    let chap_resp = client
        .get(&api_call)
        .header("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0")
        .send()
        .await?;

    let chap_content = chap_resp.text().await?;
    let chap_doc = Html::parse_document(&chap_content);
    let img_selector = Selector::parse("img").unwrap();
    let mut imgs_collection: Vec<String> = Vec::new();

    for img_link in chap_doc.select(&img_selector) {
        if let Some(img_page) = img_link.value().attr("src") {
                imgs_collection.push(img_page.to_string());
        }
    }

    if imgs_collection.is_empty() {
        println!("{}", ErrorVals::PagesNotFound);
        return Err(ErrorVals::PagesNotFound);
    }

    for (idx,page_link) in imgs_collection.iter().enumerate() {
        println!("[{}/{}] Downloading: {}", idx + 1, imgs_collection.len(), page_link);
        image_downloader(&page_link, idx, &chap_dir, data, client).await?;
    }
    complete_chapter(&chap_dir, data, chap_num)?;
    Ok(())
}


pub async fn scrape_weebcentral(data: &ScrapedData) -> Result<(), ErrorVals>
{
    let client: Client = Client::new();

    let api_url = setup_domaine_api(data);

    let web_rep = client
        .get(&api_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0")
        .header("Accept", "*/*")
        .send()
        .await?;

    let web_content = web_rep.text().await?;
    let docu = Html::parse_document(&web_content);
    let a_selector = Selector::parse("a").unwrap();

    download_cover(data).await?;

    let mut current_chaps = 0;
    for link in docu.select(&a_selector) {
        if let Some(chap_href) = link.value().attr("href") {
            if chap_href.contains("/chapters/") {
                current_chaps += 1;
            }
        }
    }
    let mut chapter_links = Vec::with_capacity(current_chaps);
    for link in docu.select(&a_selector) {
        if let Some(chap_href) = link.value().attr("href") {
            if chap_href.contains("/chapters/") {
                chapter_links.push(chap_href.to_string());
            }
        }
    }

    if chapter_links.is_empty() {
        println!("{}", ErrorVals::ChaptersNotFound);
        return  Err(ErrorVals::ChaptersNotFound);
    }

    println!("Number of chapters = {}", chapter_links.len());
    for (chap_num, link) in chapter_links.iter().rev().enumerate() {
        scrape_images(&link, data, chap_num, &client).await?;
        random_delay().await;
    }
    println!("{} has been scraped", data.title);
    Ok(())
}