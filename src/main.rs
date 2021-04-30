use teloxide::prelude::*;
use std::env;
use teloxide::types::{MessageKind, MediaKind, ParseMode};
use teloxide::net::Download;

// use log::{info, warn, debug, error, log, trace};
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    teloxide::enable_logging!();

    let mut bot = Bot::from_env();
    teloxide::repl(bot, |message| async move {
        // trace!("{:?}", message);
        match &message.update.kind {
            MessageKind::Common(msg) => {
                if let MediaKind::Document(doc) = &msg.media_kind {
                    trace!("document: {:?}", doc);
                    let id = doc.document.file_id.clone();
                    let file = message.requester.get_file(id).send().await?;
                    trace!("file: {:?}", file);

                    let mut data: Vec<u8> = vec![];
                    message.requester.download_file(&file.file_path, &mut data).await;
                    trace!("length: {}", data.len());
                    if let Ok(data) = String::from_utf8(data) {
                        trace!("\n{:?}", data);
                        message.answer("Your file:").send().await?;
                        message.answer(format!("<code>{}</code>", data))
                            .parse_mode(ParseMode::Html)
                            .send()
                            .await?;
                    }
                } else {
                    message.answer_dice().send().await?;
                }
            }
            _ => { message.answer_dice().send().await?; }
        };
        respond(())
    }).await;
    Ok(())
}