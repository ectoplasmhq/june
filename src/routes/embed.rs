use actix_web::{
    web::{self, Query},
    Responder,
};
use regex::Regex;
use serde::Deserialize;

use crate::structs::{
    embed::Embed,
    media::{Image, ImageSize, Video},
    metadata::Metadata,
};
use crate::utils::{
    request::{consume_size, fetch},
    result::Error,
};

#[derive(Deserialize)]
pub struct Parameters {
    url: String,
}

pub async fn get(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let mut url = info.into_inner().url;

    // Twitter is a piece of shit and does not provide metadata in an easily
    // consumable format.
    //
    // So, we just redirect everything to Nitter.
    //
    // Fun bonus: Twitter denied our developer application which would've been
    // the only way to pull properly-formatted tweet data out, and what's worse
    // is that this also prevents us adding those "connections" that other
    // platforms have.
    //
    // In any case, because it's Twitter, they do not provide Open Graph data.
    lazy_static! {
        static ref RE_TWITTER: Regex =
            Regex::new("^(?:https?://)?(?:www\\.)?twitter\\.com").unwrap();
    }

    if RE_TWITTER.is_match(&url) {
        url = RE_TWITTER.replace(&url, "https://nitter.soopy.moe").into();
    }

    // Fetch URL
    let (resp, mime) = fetch(&url).await?;

    // Match appropriate MIME type to process
    match (mime.type_(), mime.subtype()) {
        (_, mime::HTML) => {
            let mut metadata = Metadata::from(resp, url).await?;

            metadata.resolve_external().await;

            if metadata.is_none() {
                return Ok(web::Json(Embed::None));
            }

            Ok(web::Json(Embed::Website(metadata)))
        }
        (mime::IMAGE, _) => {
            if let Ok((width, height)) = consume_size(resp, mime).await {
                Ok(web::Json(Embed::Image(Image {
                    height,
                    size: ImageSize::Large,
                    url,
                    width,
                })))
            } else {
                Ok(web::Json(Embed::None))
            }
        }
        (mime::VIDEO, _) => {
            if let Ok((width, height)) = consume_size(resp, mime).await {
                Ok(web::Json(Embed::Video(Video { height, url, width })))
            } else {
                Ok(web::Json(Embed::None))
            }
        }
        _ => Ok(web::Json(Embed::None)),
    }
}
