use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BandcampType {
    Album,
    Track,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Special {
    None,
    GIF,
    YouTube {
        id: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        timestamp: Option<String>,
    },
    Twitch {
        id: String,
        content_type: TwitchType,
    },
    Spotify {
        id: String,
        content_type: String,
    },
    SoundCloud,
    Bandcamp {
        id: String,
        content_type: BandcampType,
    },
    Streamable {
        id: String,
    },
}

#[derive(Debug, Serialize)]
pub enum TwitchType {
    Channel,
    Video,
    Clip,
}
