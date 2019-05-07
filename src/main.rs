#[macro_use]
extern crate log;

mod telegram;
mod wallcat;

use failure::Error;

fn main() -> Result<(), Error> {
    env_logger::init();
    dotenv::dotenv()?;

    cli()
}

const AUTHOR: &str = "Sergey Sova <mail@sergeysova.com>";

fn cli() -> Result<(), Error> {
    use clap::{crate_version, App, Arg, SubCommand};

    let matches = App::new("wallcat")
        .version(crate_version!())
        .about("Post images from https://beta.wall.cat to telegram channel")
        .author(AUTHOR)
        .arg(
            Arg::with_name("channel_id")
                .help("Channel identifier: \"-1001146587123\" or @somename")
                .short("c")
                .long("channel")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("latest")
                .version(crate_version!())
                .about("Send today set of images")
                .author(AUTHOR),
        )
        .subcommand(
            SubCommand::with_name("day")
                .version(crate_version!())
                .about("Send set of images of selected day")
                .author(AUTHOR)
                .arg(
                    Arg::with_name("date")
                        .help("In format YEAR-MONTH-DAY. Example: 2019-12-01")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("latest", _) | ("", _) => publish_today(
            matches
                .value_of("channel_id")
                .expect("channel_id should be provided")
                .to_owned(),
        )?,
        ("day", Some(cmd)) => {
            let channel_id = matches
                .value_of("channel_id")
                .expect("channel_id should be provided")
                .to_owned();
            let date_value = cmd.value_of("date").expect("date should be provided");

            let datetime = chrono::NaiveDate::parse_from_str(date_value, "%Y-%m-%d")
                .expect("date should be in format %Y-%M-%d");

            let date: chrono::DateTime<chrono::Local> = chrono::DateTime::from_utc(
                datetime.and_time(chrono::NaiveTime::from_hms(0, 0, 0)),
                chrono::FixedOffset::east(0),
            );
            publish_for_a_day(channel_id, date.to_rfc3339())?
        }
        (_, _) => {}
    }

    Ok(())
}

fn publish_today(channel_id: String) -> Result<(), Error> {
    let current = chrono::Utc::now().to_rfc3339();
    publish_for_a_day(channel_id, current)
}

fn publish_for_a_day(channel_id: String, date: String) -> Result<(), Error> {
    use std::io::{stdout, Write};
    let bot_token = std::env::var("BOT_TOKEN")?;

    println!("Let's publish photos of {}\n", date);
    print!("Fetching channels from wall.cat...  ");
    stdout().flush()?;

    let bot = telegram::Telegram::new(bot_token);
    let channels = wallcat::fetch_channels()?;

    let images: Vec<_> = channels
        .iter()
        .map(|channel| wallcat::fetch_channel_image(&channel.id, &date))
        .filter_map(Result::ok)
        .collect();

    if !images.is_empty() {
        println!("OK");

        print!("Sending heading...  ");
        stdout().flush()?;

        let datetime = chrono::DateTime::parse_from_rfc3339(&date)?;
        let text = format!("<b>{}</b>", datetime.format("%d.%m.%Y"));

        bot.request(telegram::SendMessage {
            chat_id: channel_id.clone(),
            text,
            disable_notification: true,
            parse_mode: String::from("HTML"),
        })?;
        println!("OK");

        print!("Sending album...");
        stdout().flush()?;
        let mut group = telegram::SendMediaGroup::new(channel_id.clone());
        for image in &images {
            group.add_photo(telegram::InputMediaPhoto::new(
                image.url.crop(1000).clone(),
                image.channel.title.clone(),
            ));
        }
        bot.request(group)?;
        println!("OK");

        println!("Sending documents");
        for image in &images {
            print!(" - {}...  ", image.channel.title);
            stdout().flush()?;
            publish_document(&bot, &channel_id.clone(), &image)?;
            println!("OK");
        }
    } else {
        println!("Nothing");
    }

    println!("Done");

    Ok(())
}

fn publish_document<C: ToString>(
    bot: &telegram::Telegram,
    channel_id: C,
    image: &wallcat::Image,
) -> Result<(), Error> {
    let caption = create_caption(&image.channel.title);

    bot.request(telegram::SendDocument {
        chat_id: channel_id.to_string(),
        document: image.url.original.to_string(),
        caption: Some(caption),
        thumb: Some(image.url.small.to_string()),
    })?;

    Ok(())
}

fn create_caption<T: AsRef<str>>(title: T) -> String {
    format!("#{}", title.as_ref().replace(" ", ""))
}
