
use crate::{ErrorVals, ScrapedData};

use crate::components::{
    mangapill::handle_mangapill::scrape_mangapill,
    weebcentral::handle_weebcentral::scrape_weebcentral,
    mangadex::handle_mangadex::scrape_mangadex,
};


pub async fn which_scraper(data: &ScrapedData) -> Result<(), ErrorVals>
{
    match data.website.as_str() {
        "weebcentral.com" => {
            scrape_weebcentral(data).await?;
        }
        "mangadex.org" => {
            scrape_mangadex(data).await?;
        }
        "mangapill.com" => {
            scrape_mangapill(data).await?;
        }
        &_ => {}
    }
    Ok(())
}
