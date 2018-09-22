use failure::Error;
use reqwest;

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
    #[fail(display = "Network error")]
    Network,

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
    pub s: String,
    pub m: String,
    pub l: String,
    pub o: String,
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
}

#[derive(Debug, Deserialize)]
pub struct ImagePayload {
    pub image: Image,
}

const API: &'static str = "https://beta.wall.cat/api/v1";

pub fn fetch_channels() -> Result<Vec<Channel>, Error> {
    info!("Fetching channels");
    let list: OptionalResponse<Vec<Channel>> =
        reqwest::get(format!("{}/channels", API).as_str())?.json()?;

    debug!("Fetched channels: {:#?}", list);

    list.into()
}

pub fn fetch_channel_image(channel_id: String, datetime: String) -> Result<Image, Error> {
    info!("Fetching image in channel {id} for {date}", id=channel_id, date=datetime);
    let image: OptionalResponse<ImagePayload> = reqwest::get(
        format!(
            "{api}/channels/{id}/image/{date}",
            api = API,
            id = channel_id,
            date = datetime
        ).as_str(),
    )?.json()?;

    let image: Result<ImagePayload, Error> = image.into();

    debug!("Fetched image: {:#?}", image);

    image.and_then(|payload| Ok(payload.image))
}
