use failure::{Error, Fail};
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub payload: T,
}

#[derive(Debug, Deserialize)]
pub struct ErrorPayload {
    message: String,
}

#[derive(Debug, Fail)]
pub enum WallcatError {
    // #[fail(display = "Network error")]
    // Network,
    #[fail(display = "Bad request: {}", reason)]
    BadRequest { reason: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OptionalResponse<T> {
    Fail(Response<ErrorPayload>),
    Success(Response<T>),
}

impl<T> Into<Result<T, Error>> for OptionalResponse<T> {
    fn into(self) -> Result<T, Error> {
        Ok(match self {
            OptionalResponse::Fail(err) => Err(WallcatError::BadRequest {
                reason: err.payload.message,
            }),
            OptionalResponse::Success(value) => Ok(value.payload),
        }?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct UrlMap {
    /// 20px
    #[serde(rename = "s")]
    pub small: String,
    /// 100px
    #[serde(rename = "m")]
    pub middle: String,
    /// 500px
    #[serde(rename = "l")]
    pub large: String,
    /// maximum size
    #[serde(rename = "o")]
    pub original: String,
}

impl UrlMap {
    pub fn crop(&self, width: u16) -> String {
        format!("{}?crop=fit&w={}", self.original, width)
    }
}

#[derive(Debug, Deserialize)]
pub struct ActiveDate {
    /// 2018-10-10T00:00:00.000Z
    pub raw: String,
    /// Oct 10th 2018
    pub normalized: String,
    /// 2018-10-10
    pub calendar: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: String,
    pub channel: Channel,
    pub title: String,
    pub url: UrlMap,
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
    #[serde(rename = "webLocation")]
    pub web_location: String,
    #[serde(rename = "activeDate")]
    pub active_date: ActiveDate,
}

#[derive(Debug, Deserialize)]
pub struct ImagePayload {
    pub image: Image,
}

const API: &str = "https://beta.wall.cat/api/v1";

pub fn fetch_channels() -> Result<Vec<Channel>, Error> {
    info!("Fetching channels");
    let list: OptionalResponse<Vec<Channel>> =
        reqwest::get(format!("{}/channels", API).as_str())?.json()?;

    debug!("Fetched channels: {:#?}", list);

    list.into()
}

fn url_of_image(channel_id: &str, datetime: &str) -> String {
    format!(
        "{api}/channels/{id}/image/{date}",
        api = API,
        id = channel_id,
        date = datetime
    )
}

pub fn fetch_channel_image<I, D>(channel_id: I, datetime: D) -> Result<Image, Error>
where
    I: AsRef<str>,
    D: AsRef<str>,
{
    let channel_id = channel_id.as_ref();
    let datetime = datetime.as_ref();
    info!(
        "Fetching image in channel {id} for {date}",
        id = channel_id,
        date = datetime
    );
    let url = url_of_image(channel_id, datetime);
    let image: OptionalResponse<ImagePayload> = reqwest::get(url.as_str())?.json()?;

    let image: Result<ImagePayload, Error> = image.into();

    debug!("Fetched image: {:#?}", image);

    image.and_then(|payload| Ok(payload.image))
}
