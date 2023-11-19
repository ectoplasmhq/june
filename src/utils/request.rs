use mime::Mime;
use reqwest::{header::CONTENT_TYPE, Client, Response};
use scraper::Html;
use std::io::Write;
use std::time::Duration;
use tempfile::NamedTempFile;

use super::result::Error;

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; June/0.1; +https://github.com/ectoplasmhq/june)")
        .timeout(Duration::from_secs(2))
        .build()
        .expect("reqwest Client");
}

pub async fn consume_fragment(resp: Response) -> Result<Html, Error> {
    let body = resp.text().await.map_err(|_| Error::FailedToConsumeText)?;

    Ok(Html::parse_document(&body))
}

pub async fn consume_size(
    resp: Response,
    mime: Mime,
) -> Result<(isize, isize), Error> {
    let bytes = resp
        .bytes()
        .await
        .map_err(|_| Error::FailedToConsumeBytes)?;

    match mime.type_() {
        mime::IMAGE => {
            if let Ok(size) = imagesize::blob_size(&bytes) {
                Ok((size.width as isize, size.height as isize))
            } else {
                Err(Error::CouldNotDetermineImageSize)
            }
        }
        mime::VIDEO => {
            let mut tmp = NamedTempFile::new()
                .map_err(|_| Error::CouldNotDetermineVideoSize)?;

            tmp.write_all(&bytes)
                .map_err(|_| Error::CouldNotDetermineVideoSize)?;

            determine_video_size(tmp.path())
        }
        _ => unreachable!(),
    }
}

pub fn determine_video_size(
    path: &std::path::Path,
) -> Result<(isize, isize), Error> {
    let data = ffprobe::ffprobe(path).map_err(|_| Error::ProbeError)?;

    // Take the first valid stream
    for stream in data.streams {
        if let (Some(w), Some(h)) = (stream.width, stream.height) {
            if let (Ok(w), Ok(h)) = (w.try_into(), h.try_into()) {
                return Ok((w, h));
            }
        }
    }

    Err(Error::ProbeError)
}

pub async fn fetch(url: &str) -> Result<(Response, Mime), Error> {
    let resp = CLIENT
        .get(url)
        .send()
        .await
        .map_err(|_| Error::ReqwestFailed)?;

    if !resp.status().is_success() {
        return Err(Error::RequestFailed);
    }

    let content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .ok_or(Error::MissingContentType)?
        .to_str()
        .map_err(|_| Error::ConversionFailed)?;

    let mime: mime::Mime = content_type
        .parse()
        .map_err(|_| Error::FailedToParseContentType)?;

    Ok((resp, mime))
}
