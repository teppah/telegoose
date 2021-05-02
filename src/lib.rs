pub mod states;

use crate::states::{StartState, ReceiveFileState, ReceiveFormatState};
use teloxide::macros::Transition;

#[macro_use]
extern crate log;

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
