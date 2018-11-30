extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate chrono;

mod date;
mod telegram;
mod wallcat;

use failure::Error;

fn main() -> Result<(), Error> {
    env_logger::init();
    dotenv::dotenv()?;

    let bot_token = std::env::var("BOT_TOKEN")?;
    let chat_id = std::env::var("CHAT_ID")?;

    let bot = telegram::Telegram::new(bot_token);
    let channels = wallcat::fetch_channels()?;
    let current_date = date::now_utc();

    bot.request(telegram::SendMessage {
        chat_id: chat_id.clone(),
        text: format!("{}", date::now_readable()),
        disable_notification: true,
    }).expect("Message should be sended");

    for channel in channels.into_iter() {
        let image = wallcat::fetch_channel_image(channel.id, current_date.clone())?;
        let caption = "#".to_string() + &image.channel.title.replace(" ", "");

        println!(
            "#{}: {} -> {}",
            image.channel.id, image.channel.title, image.url.original
        );

        bot.request(telegram::SendPhoto {
            chat_id: chat_id.clone(),
            photo: image.url.large,
            caption: Some(caption.clone()),
        }).expect("Photo should be sended");

        bot.request(telegram::SendDocument {
            chat_id: chat_id.clone(),
            document: image.url.original,
            caption: Some(caption.clone()),
            thumb: Some(image.url.small),
        }).expect("Document should be sended");
    }

    Ok(())
}
