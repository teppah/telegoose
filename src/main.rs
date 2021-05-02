use teloxide::prelude::*;
use std::env;
use teloxide::types::{MessageKind, MediaKind};

#[macro_use]
extern crate log;

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

    teloxide::dialogues_repl(bot, |cx, dialogue: Dialogue| async move {
        handle_message(cx, dialogue).await.unwrap_or_else(|e| {
            error!("Request failed, Telegram error: {:?}", e);
            panic!("Request failed")
        })
    }).await;
}

// FSM state transition logic
async fn handle_message(cx: UpdateWithCx<Bot, Message>, dialogue: Dialogue) -> TransitionOut<Dialogue> {
    trace!("{:?}", cx.update);
    match &dialogue {
        Dialogue::ReceiveFile(_) => {
            match &cx.update.kind {
                MessageKind::Common(ref c)
                if matches!(c.media_kind, MediaKind::Document(_)) => {
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