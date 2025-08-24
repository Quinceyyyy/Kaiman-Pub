use reqwest::{Client, Url};
use scraper::{self, Selector, Html};
use tokio::time::{sleep, Duration};
use std::path::PathBuf;

use crate::{
    utils::{
        image_helpers::{image_downloader},
        chapter_dir_helpers::{write_chap_dir, complete_chapter, check_completed_marker},
        misc::{random_delay},
    },
    errors::ErrorVals, 
    ScrapedData,
};


async fn scrape_imgs(chapter_link: &str, data: &ScrapedData, chap_num: usize, target_client: &Client) -> Result<(), ErrorVals>
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
    let web_url = Url::parse("https://mangapill.com").unwrap();
    let completed_url = web_url.join(&chapter_link).unwrap();

    let chap_resp = target_client
        .get(completed_url)
        .send()
        .await?;

    let chap_content = chap_resp.text().await?;
    let chap_doc = Html::parse_document(&chap_content);
    let img_selector = Selector::parse("img.js-page").unwrap();
    let mut imgs_collection: Vec<String> = Vec::new();

    for img_link in chap_doc.select(&img_selector) {
        if let Some(img_page) = img_link.value().attr("data-src") {
                imgs_collection.push(img_page.to_string());
        }
    }
    for (idx,page_link) in imgs_collection.iter().enumerate() {
        println!("downloading: {}", page_link);
        image_downloader(&page_link, idx, &chap_dir, data, target_client).await?;
        sleep(Duration::from_millis(500)).await;
    }
    complete_chapter(&chap_dir, data, chap_num)?;
    Ok(())
}


pub async fn scrape_mangapill(data: &ScrapedData) -> Result<(), ErrorVals>
{
    let target_client: Client = Client::builder()
        .cookie_store(true)
        .build()?;

    let target_resp = target_client
        .get(&data.input_url)
        .send()
        .await?;

    let target_content = target_resp.text().await?;
    let docu = Html::parse_document(&target_content);
    let a_selector = Selector::parse("a").unwrap();

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
    for (chap_num, chapter_link )in chapter_links.iter().rev().enumerate() {
        scrape_imgs(chapter_link, data, chap_num, &target_client).await?;
        random_delay().await;
    }
    Ok(())
}