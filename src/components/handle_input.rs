use url::Url;
use reqwest::StatusCode;

use crate::{components::{handle_library::setup_library, handle_scraping::which_scraper}, ErrorVals, ScrapedData};



async fn prob_link(data: &ScrapedData) -> Result<(), ErrorVals>
{
    let client = reqwest::Client::new();

    let prob_url = match data.website.as_str(){
        "mangadex.org" => { format!("https://api.mangadex.org/manga/{}", data.series_id)},
        _ => data.input_url.clone(),
    };

    let rep = client
        .get(prob_url)
        .header("User-Agent", "Kaimanv2.0 (https://github.com/Quinceyyyy/Kaiman)")
        .send()
        .await.map_err(ErrorVals::HttpError)?;

    match rep.status() {
        StatusCode::OK => {println!("{} page found !", data.title); Ok(())},
        StatusCode::NOT_FOUND => Err(ErrorVals::SeriesNotFound),
        status => {
            let error_msg = rep.text().await.unwrap_or_else(|_| "<no bodyy>".to_string());
            eprintln!("Status code: {}/nBody: {}", status, error_msg); 
            Err(ErrorVals::SurpriseError)
        },
    }
}


async fn which_domain(data: &mut ScrapedData, url_splits: &Vec<&str>) -> Result<(), ErrorVals>
{
    match data.website.as_str() {
        "weebcentral.com" => {
            if url_splits.len() == 3 && url_splits[0] == "series" {
                data.series_id = url_splits[1].to_string();
                data.title = url_splits[2].to_string();
            }
        }
        "mangadex.org" => {
            if url_splits.len() == 3 && url_splits[0] == "title" {
                data.series_id = url_splits[1].to_string();
                data.title = url_splits[2].to_string();
            }
        }
        "mangapill.com" => {
            if url_splits.len() == 3 && url_splits[0] == "manga" {
                data.series_id = url_splits[1].to_string();
                data.title = url_splits[2].to_string();
            }
        }
        &_ => {
            return Err(ErrorVals::InvalidWebsite);
        }
    }
    prob_link(data).await?;
    setup_library(data).await?;
    which_scraper(data).await?;

    Ok(())
}

pub async fn pre_scrape_setup(data: &mut ScrapedData) -> Result<(), ErrorVals>
{
    let parsed_input = Url::parse(&data.input_url).map_err(|_| ErrorVals::NoURL)?;
    let website = parsed_input.domain().ok_or(ErrorVals::InvalidURL)?;
    data.website = website.to_string();

    let url_splits: Vec<&str> = parsed_input
        .path_segments()
        .map(|c| c.collect())
        .unwrap_or_else(Vec::new);

    which_domain(data, &url_splits).await?;
    Ok(())
}
