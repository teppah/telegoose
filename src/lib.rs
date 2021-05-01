use teloxide::prelude::*;
use teloxide::macros::Transition;
use teloxide::types::{MediaDocument, MediaKind, Document, MessageKind, InputFile};

#[macro_use]
extern crate log;

// use log::{info, warn, debug, error, log, trace};
use crate::Dialogue::ReceiveFormat;
use teloxide::net::Download;
use reqwest::Method;
use std::borrow::Borrow;

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
async fn start(state: StartState, cx: TransitionIn<Bot>, _: String) -> TransitionOut<Dialogue> {
    cx.answer("Send a PDF file to be processed.").send().await?;
    next(ReceiveFileState)
}

#[derive(Clone)]
pub struct ReceiveFileState;

#[teloxide(subtransition)]
async fn receive_file(state: ReceiveFileState, cx: TransitionIn<Bot>, _: String) -> TransitionOut<Dialogue> {
    // assume that there is a MediaDocument sent
    let doc = match cx.update.kind {
        MessageKind::Common(ref c) => {
            match &c.media_kind {
                MediaKind::Document(doc) => {
                    let MediaDocument { document, .. } = doc;
                    document
                }
                _ => panic!("This should not happen")
            }
        }
        _ => panic!("This should not happen")
    };
    let Document { file_id, .. } = doc;

    if doc.mime_type == Some(mime::APPLICATION_PDF) {
        cx.answer("File received, what is the format?").send().await?;
        trace!("File id: {}", file_id);
        next(ReceiveFormatState { file_id: file_id.clone() })
    } else {
        cx.answer("Sorry, this bot only supports PDF files for now. Please send a PDF file.").send().await?;
        next(state)
    }
}

#[derive(Clone)]
pub struct ReceiveFormatState {
    file_id: String,
}

#[teloxide(subtransition)]
async fn receive_format(state: ReceiveFormatState, cx: TransitionIn<Bot>, format: String) -> TransitionOut<Dialogue> {
    // assume existence
    let url = std::env::var("GOOSE_URL").unwrap();

    cx.answer("Processing...").send().await?;
    trace!("Format: {}", format);

    let file = cx.requester.get_file(&state.file_id).send().await?;
    let mut data: Vec<u8> = vec![];
    cx.requester.download_file(&file.file_path, &mut data).await;

    let client = match reqwest::Client::builder().brotli(true).build() {
        Ok(c) => c,
        Err(e) => {
            error!("Error building client: {}", e);
            cx.answer(format!("Error, please start over: {}", e)).send().await?;
            return next(StartState);
        }
    };

    let length = data.len();
    let pdf_part = reqwest::multipart::Part::stream_with_length(data, length as u64)
        .file_name("file.pdf")
        .mime_str("application/pdf").unwrap();
    let form = reqwest::multipart::Form::new()
        .text("formatString", format)
        .part("file", pdf_part);

    let res = client.request(Method::POST, url)
        .multipart(form)
        .send().await.unwrap();
    let new_data = res.bytes().await.unwrap();

    let new_file = InputFile::Memory { file_name: "Split.zip".to_string(), data: new_data.to_vec().into() };
    cx.answer("Here is your processed thing!").send().await?;
    cx.answer_document(new_file).caption("Here is your file!").send().await?;
    next(StartState)
}
