use teloxide::prelude::*;
use teloxide::macros::Transition;

use log::{info, warn, debug, error, log, trace};
use crate::Dialogue::ReceiveFormat;

#[derive(Transition, Clone)]
pub enum Dialogue {
    Start(StartState),
    ReceiveFile(ReceiveFileState),
    ReceiveFormat(ReceiveFormatState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}

// TODO: use derive_more
impl ::std::convert::From<StartState> for Dialogue {
    fn from(start: StartState) -> Self {
        Dialogue::Start(start)
    }
}

impl ::std::convert::From<ReceiveFileState> for Dialogue {
    fn from(receive_file: ReceiveFileState) -> Self {
        Dialogue::ReceiveFile(receive_file)
    }
}

impl ::std::convert::From<ReceiveFormatState> for Dialogue {
    fn from(receive_format: ReceiveFormatState) -> Self {
        Dialogue::ReceiveFormat(receive_format)
    }
}

#[derive(Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(state: StartState, cx: TransitionIn<Bot>, ans: String) -> TransitionOut<Dialogue> {
    cx.answer("Send a PDF file to be processed.").send().await?;
    next(ReceiveFileState)
}

#[derive(Clone)]
pub struct ReceiveFileState;

#[teloxide(subtransition)]
async fn receive_file(state: ReceiveFileState, cx: TransitionIn<Bot>, ans: String) -> TransitionOut<Dialogue> {
    cx.answer("File received, what is the format?").send().await?;
    next(ReceiveFormatState)
}

#[derive(Clone)]
pub struct ReceiveFormatState;

#[teloxide(subtransition)]
async fn receive_format(state: ReceiveFormatState, cx: TransitionIn<Bot>, ans: String) -> TransitionOut<Dialogue> {
    cx.answer("Here is your processed thing!").send().await?;
    next(StartState)
}
