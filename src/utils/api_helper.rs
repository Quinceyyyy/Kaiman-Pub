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

pub fn create_api_call(chapter_link: &str) -> Result<String, ErrorVals>
{
    let chap_id = chapter_link.rsplit('/').next().ok_or(ErrorVals::InvalidURL)?;
    let weebcentral_api = format!("https://weebcentral.com/chapters/{}/images?is_prev=False&current_page=1&reading_style=long_strip", chap_id);

    Ok(weebcentral_api)
}
