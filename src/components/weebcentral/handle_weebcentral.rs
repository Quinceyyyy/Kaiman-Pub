use reqwest::Client;
use scraper::{self, Selector, Html};

use crate::{components
    ::utils::{
    create_api_call,
    image_downloader, 
    write_chap_dir,
    setup_domaine_api,
    random_delay
    },
    errors::ErrorVals, ScrapedData
};


async fn scrape_images(chapter_link: &str, data: &ScrapedData, chap_num: usize) -> Result<(), ErrorVals>
{
    let chap_dir = match write_chap_dir(data, chap_num)? {
        Some(chap_dir) => chap_dir,
        None => {
            println!("Skipping chapter {}: it alreadt exists", chap_num + 1);
            return Ok(());
        }
    };

    let api_call = create_api_call(chapter_link, data)?;

    let page_client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0")
        .build()?;

    let chap_resp = page_client
        .get(api_call)
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
    for (idx,page_link) in imgs_collection.iter().enumerate() {
        println!("downloading: {}", page_link);
        image_downloader(&page_link, idx, &chap_dir).await?;
    }
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
        scrape_images(&link, data, chap_num).await?;
        random_delay().await;
    }
    println!("{} has been scraped", data.title);
    Ok(())
}