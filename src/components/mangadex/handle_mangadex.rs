use reqwest::Client;
use tokio::time::{sleep, Duration};
use serde::Deserialize;

use crate::{components
    ::utils::{
    image_downloader, 
    write_chap_dir,
    setup_domaine_api,
    },
    errors::ErrorVals, ScrapedData
};

#[derive(Deserialize, Debug)]
struct ChapterListResp {
    data: Vec<ChapterRes>,
}

#[derive(Deserialize, Debug)]
struct ChapterRes {
    id: String,
    #[serde(rename = "attributes")]
    _attributes: ChapterAttributes,
}

#[derive(Deserialize, Debug)]
struct ChapterAttributes {
    #[serde(rename = "chapter")]
    _chapter: Option<String>,
    #[serde(rename = "title")]
    _title: Option<String>,
}

#[derive(Deserialize, Debug)]
struct HomeResp {
    #[serde(rename = "baseUrl")]
    base_url: String,
    chapter: HomeChapter,
}

#[derive(Deserialize, Debug)]
struct HomeChapter {
    hash: String,
    data: Vec<String>,
}


async fn scrape_images(home_rep: HomeResp, data: &ScrapedData, chap_num: usize) -> Result<(), ErrorVals>
{
    let chap_dir = match write_chap_dir(data, chap_num)? {
        Some(chap_dir) => chap_dir,
        None => {
            println!("Skipping chapter {}: it alreadt exists", chap_num + 1);
            return Ok(());
        }
    };

    for (idx, page) in home_rep.chapter.data.iter().enumerate() {
        let img_url = format!("{}/data/{}/{}", home_rep.base_url, home_rep.chapter.hash, page);
        println!("downloading: {}", img_url);
        image_downloader(&img_url, idx, &chap_dir).await?;
    }
    Ok(())
}


pub async fn scrape_mangadex(data: &ScrapedData) -> Result<(), ErrorVals>
{
    let client: Client = Client::new();
    let api_url = setup_domaine_api(data);
    println!("{api_url}");

    let chapters_rep = client
        .get(&api_url)
        .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
        .send()
        .await?
        .json::<ChapterListResp>()
        .await?;

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
    
        scrape_images(home_rep, data, idx).await?;
        sleep(Duration::from_millis(1000)).await;
    }

    println!("{} has been scraped", data.title);
    Ok(())
}