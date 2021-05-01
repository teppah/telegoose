use teloxide::prelude::*;
use std::env;
use teloxide::types::{MessageKind, MediaKind, ParseMode, DiceEmoji};
use teloxide::net::Download;

// use log::{info, warn, debug, error, log, trace};
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    run().await;
}

async fn run() {
    let bot = Bot::from_env();
    info!("Bot started");
    teloxide::repl(bot, |message| async move {
        match &message.update.kind {
            MessageKind::Common(msg) => {
                if let MediaKind::Document(doc) = &msg.media_kind {
                    trace!("{:?}", doc.document);
                    let id = doc.document.file_id.clone();
                    let file = message.requester.get_file(id).send().await?;
                    trace!("{:?}", file);
                    let mut data: Vec<u8> = vec![];
                    message.requester.download_file(&file.file_path, &mut data).await;
                    trace!("length: {}", data.len());
                    message.answer("Received your file.").send().await?;
                } else {
                    message.answer_dice().send().await?;
                }
            }
            _ => { message.answer_dice().send().await?; }
        };
        respond(())
    }).await;
}

