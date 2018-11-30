use serde::Serialize;

pub trait TelegramMethod {
    fn method() -> String;
}

#[derive(Debug, Serialize)]
pub struct SendDocument {
    /// nique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: String,

    /// File to send.
    /// Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data.
    pub document: String,

    /// Thumbnail of the file sent.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and heightshould not exceed 90.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded
    /// using multipart/form-data under <file_attach_name>
    pub thumb: Option<String>,

    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters
    pub caption: Option<String>,
}

impl TelegramMethod for SendDocument {
    fn method() -> String {
        "sendDocument".to_string()
    }
}

#[derive(Debug, Serialize)]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: String,

    /// Photo to send.
    /// Pass a file_id as String to send a photo that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a photo from the Internet,
    /// or upload a new photo using multipart/form-data
    pub photo: String,

    /// Photo caption (may also be used when resending photos by file_id), 0-1024 characters
    pub caption: Option<String>,
}

impl TelegramMethod for SendPhoto {
    fn method() -> String {
        "sendPhoto".to_string()
    }
}

#[derive(Debug, Serialize)]
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: String,

    /// Text of the message to be sent
    pub text: String,

    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: bool,
}

impl TelegramMethod for SendMessage {
    fn method() -> String {
        "sendMessage".to_string()
    }
}

pub struct Telegram {
    bot_token: String,
}

impl Telegram {
    pub fn new<T: Into<String>>(token: T) -> Self {
        Telegram {
            bot_token: token.into(),
        }
    }

    pub fn request<B: Serialize + TelegramMethod>(
        &self,
        body: B,
    ) -> reqwest::Result<reqwest::Response> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.telegram.org/bot{}/{}",
            self.bot_token,
            B::method()
        );
        client.post(url.as_str()).json(&body).send()
    }
}
