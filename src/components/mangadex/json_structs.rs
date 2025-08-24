use serde::Deserialize;




#[derive(Deserialize, Debug)]
pub struct ChapterListResp {
    pub data: Vec<ChapterRes>,
}

#[derive(Deserialize, Debug)]
pub struct ChapterRes {
   pub id: String,
    #[serde(rename = "attributes")]
    _attributes: ChapterAttributes,
}

#[derive(Deserialize, Debug)]
pub struct ChapterAttributes {
    #[serde(rename = "chapter")]
    _chapter: Option<String>,
    #[serde(rename = "title")]
    _title: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct HomeResp {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: HomeChapter,
}

#[derive(Deserialize, Debug)]
pub struct HomeChapter {
    pub hash: String,
    pub data: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct MangadexResp {
    pub data: MangaData,
}

#[derive(Deserialize, Debug)]
pub struct MangaData {
    #[serde(rename = "id")]
   _id: String,
   pub relationships: Vec<Relationship>,
}

#[derive(Deserialize, Debug)]
pub struct Relationship {
    #[serde(rename= "type")]
    pub rel_type: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct CoverResp {
    pub data: CoverData,
}

#[derive(Deserialize, Debug)]
pub struct CoverData {
    pub attributes: CoverAttributes,
}

#[derive(Deserialize, Debug)]
pub struct CoverAttributes {
    #[serde(rename= "fileName")]
    pub filename: String,
}