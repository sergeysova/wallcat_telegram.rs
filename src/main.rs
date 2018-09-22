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
mod wallcat;

use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    env_logger::init();
    dotenv::dotenv()?;

    println!("Hello, world!");

    let channels = wallcat::fetch_channels()?;
    let date = date::now_utc();

    for channel in channels.into_iter() {
        let image = wallcat::fetch_channel_image(channel.id, date.clone())?;
        // println!("{:#?}", image);
    }

    Ok(())
}
