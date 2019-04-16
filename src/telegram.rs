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
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: String,

    /// Text of the message to be sent
    pub text: String,

    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: bool,

    /// Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in your bot's message.
    pub parse_mode: String,
}

impl TelegramMethod for SendMessage {
    fn method() -> String {
        "sendMessage".to_string()
    }
}

/// Use this method to send a native poll. A native poll can't be sent to a private chat.
/// On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername).
    /// A native poll can't be sent to a private chat.
    pub chat_id: String,

    /// Poll question, 1-255 characters
    pub question: String,

    /// List of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,

    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: bool,

    /// Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in your bot's message.
    pub parse_mode: String,
}

impl TelegramMethod for SendPoll {
    fn method() -> String {
        "sendPoll".to_string()
    }
}

impl SendPoll {
    pub fn new<C, Q>(chat_id: C, question: Q) -> Self
    where
        C: Into<String>,
        Q: Into<String>,
    {
        SendPoll {
            chat_id: chat_id.into(),
            question: question.into(),
            options: vec![],
            disable_notification: false,
            parse_mode: "HTML".to_string(),
        }
    }

    pub fn add_option(&mut self, option: String) -> &mut Self {
        self.options.push(option);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername).
    /// A native poll can't be sent to a private chat.
    pub chat_id: String,

    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub media: Vec<InputMediaPhoto>,

    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: bool,
}

impl TelegramMethod for SendMediaGroup {
    fn method() -> String {
        "sendMediaGroup".to_string()
    }
}

impl SendMediaGroup {
    pub fn new<I: Into<String>>(chat_id: I) -> Self {
        SendMediaGroup {
            chat_id: chat_id.into(),
            media: vec![],
            disable_notification: false,
        }
    }

    pub fn add_photo(&mut self, photo: InputMediaPhoto) -> &mut Self {
        self.media.push(photo);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct InputMediaPhoto {
    /// Type of the result, must be `photo`
    #[serde(rename = "type")]
    pub photo_type: String,

    /// File to send.
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data under <file_attach_name> name.
    pub media: String,

    /// Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,

    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: String,
}

impl InputMediaPhoto {
    pub fn new(media: String, caption: String) -> Self {
        InputMediaPhoto {
            photo_type: "photo".to_string(),
            media,
            caption: Some(caption),
            parse_mode: "HTML".to_string(),
        }
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
