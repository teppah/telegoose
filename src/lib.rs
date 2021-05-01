use teloxide::prelude::*;

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

#[derive(Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(state: StartState, cx: TransitionIn<AutoSend<Bot>>) -> TransitionOut<Dialogue> {}

#[derive(Clone)]
pub struct ReceiveFileState;

#[teloxide(subtransition)]
async fn receive_file(state: ReceiveFileState, cx: TransitionIn<AutoSend<Bot>>) -> TransitionOut<Dialogue> {}

#[derive(Clone)]
pub struct ReceiveFormatState;

#[teloxide(subtransition)]
async fn receive_format(state: ReceiveFormatState, cx: TransitionIn<AutoSend<Bot>>) -> TransitionOut<Dialogue> {}
