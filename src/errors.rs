use reqwest;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum ErrorVals {
    NoURL,
    InvalidWebsite,
    InvalidURL,
    InvalidPath,
    SeriesNotFound,
    SurpriseError,
    ChaptersNotFound,
    PagesNotFound,
    CoverNotFound,
    HttpError(reqwest::Error),
    IoError(io::Error),
}

impl From<reqwest::Error> for ErrorVals {
    fn from(err: reqwest::Error) -> Self {
        ErrorVals::HttpError(err)
    }
}

impl From<io::Error> for ErrorVals {
    fn from(err: io::Error) -> Self {
        ErrorVals::IoError(err)
    }
}

impl fmt::Display for ErrorVals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorVals::HttpError(e) => write!(f, "HTTP Error: {}", e),
            ErrorVals::IoError(e) => write!(f, "IO Error: {}", e),
            ErrorVals::InvalidWebsite => write!(f, "Invalid website, not a Weebcentral link"),
            ErrorVals::SeriesNotFound => write!(f, "Series unavailable"),
            ErrorVals::NoURL => write!(f, "Please add a URL from mangadex or weebcentral"),
            ErrorVals::InvalidURL => write!(f, "Invalid URL"),
            ErrorVals::SurpriseError => write!(f, "Unexpected Error"),
            ErrorVals::ChaptersNotFound => write!(f, "No chapters found"),
            ErrorVals::InvalidPath => write!(f, "Input path doesn't exist"),
            ErrorVals::PagesNotFound => write!(f, "Error finding chapter pages"),
            ErrorVals::CoverNotFound => write!(f, "Error finding cover art"),
        }
    }
}
