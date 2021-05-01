use teloxide::prelude::*;
use std::env;
use teloxide::types::{MessageKind, MediaKind, ParseMode, DiceEmoji,MediaDocument, Document};
use teloxide::net::Download;

#[macro_use]
extern crate log;
// use log::{info, warn, debug, error, log, trace};
use telegoose::Dialogue;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    // check existence of environment variables
    match (std::env::var("GOOSE_URL"), std::env::var("TELOXIDE_TOKEN")) {
        (Ok(_), Ok(_)) => {}
        _ => {
            error!("Missing/invalid environment variables: GOOSE_URL and/or TELOXIDE_TOKEN");
            std::process::exit(1);
        }
    }
    run().await;
}

async fn run() {
    let bot = Bot::from_env();
    info!("Bot started");
    // teloxide::repl(bot, |message| async move {
    //     match &message.update.kind {
    //         MessageKind::Common(msg) => {
    //             if let MediaKind::Document(doc) = &msg.media_kind {
    //                 trace!("{:?}", doc.document);
    //                 let id = doc.document.file_id.clone();
    //                 let file = message.requester.get_file(id).send().await?;
    //                 trace!("{:?}", file);
    //                 let mut data: Vec<u8> = vec![];
    //                 message.requester.download_file(&file.file_path, &mut data).await;
    //                 trace!("length: {}", data.len());
    //                 message.answer("Received your file.").send().await?;
    //             } else {
    //                 message.answer_dice().send().await?;
    //             }
    //         }
    //         _ => { message.answer_dice().send().await?; }
    //     };
    //     respond(())
    // }).await;

    teloxide::dialogues_repl(bot, |cx, dialogue: Dialogue| async move {
        handle_message(cx, dialogue).await.expect("Something went wrong with the bot")
    }).await;
}


// FSM state transition logic
async fn handle_message(cx: UpdateWithCx<Bot, Message>, dialogue: Dialogue) -> TransitionOut<Dialogue> {
    match &dialogue {
        Dialogue::ReceiveFile(_) => {
            match &cx.update.kind {
                MessageKind::Common(ref c)
                    if matches!(c.media_kind, MediaKind::Document(_))=> {
                    dialogue.react(cx, "".into()).await
                }
                _ => {
                    cx.answer("Please send a file to continue.").send().await?;
                    next(dialogue)
                }
            }
        }
        Dialogue::Start(_) | Dialogue::ReceiveFormat(_) => {
            if let Some(s) = cx.update.text().map(|s| s.to_string()) {
                dialogue.react(cx, s).await
            } else {
                // fallback FSM transition
                cx.answer("Please send a message to continue.").send().await?;
                next(dialogue)
            }
        }
    }
}