use teloxide::prelude::*;
use crate::Dialogue;
use teloxide::types::{InputFile, MessageKind, MediaKind, MediaDocument, Document};
use teloxide::net::Download;
use reqwest::{Client, Method};
use reqwest::multipart::{Part, Form};

use log::{warn, debug};

#[derive(Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(_state: StartState, cx: TransitionIn<Bot>, _: String) -> TransitionOut<Dialogue> {
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

const ERROR_MSG: &'static str = "Oops, something went wrong. Please restart.";
const WRONG_INPUT_MSG: &'static str = "Oops, something was wrong with your input, check your input file and the format. Make sure the total number of pages is correct and try again.";
const SERVER_ERROR_MSG: &'static str = "Oops, something went wrong on the server-side. Make sure your file is a valid PDF file and try again.";

#[teloxide(subtransition)]
async fn receive_format(state: ReceiveFormatState, cx: TransitionIn<Bot>, format: String) -> TransitionOut<Dialogue> {
    // assume existence
    let url = std::env::var("GOOSE_URL").unwrap();

    cx.answer("Processing...").send().await?;

    let file = cx.requester.get_file(&state.file_id).send().await?;
    let mut data: Vec<u8> = vec![];
    match cx.requester.download_file(&file.file_path, &mut data).await {
        Err(e) => {
            error!("Failed to download telegram file: {:?}", e);
            cx.answer(ERROR_MSG).send().await?;
            return next(StartState);
        }
        _ => ()
    };

    let client = match Client::builder().brotli(true).build() {
        Ok(c) => c,
        Err(e) => {
            error!("Error building client: {}", e);
            cx.answer(ERROR_MSG).send().await?;
            return next(StartState);
        }
    };

    let length = data.len();
    let pdf_part = match Part::stream_with_length(data, length as u64)
        .file_name("file.pdf")
        .mime_str("application/pdf") {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to construct multipart request: {}", e);
            cx.answer(ERROR_MSG).send().await?;
            return next(StartState);
        }
    };
    let form = Form::new()
        .text("formatString", format.to_string())
        .part("file", pdf_part);

    let res = match client.request(Method::POST, url)
        .multipart(form)
        .send().await {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to send request: {}", e);
            cx.answer(ERROR_MSG).send().await?;
            return next(StartState);
        }
    };

    if res.status().is_success() {
        let new_data = match res.bytes().await {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to retrieve response bytes: {}", e);
                cx.answer(ERROR_MSG).send().await?;
                return next(StartState);
            }
        };

        let new_file = InputFile::Memory { file_name: "Questions.zip".to_string(), data: new_data.to_vec().into() };
        cx.answer_document(new_file).caption("Here is your file!").send().await?;
        debug!("Processed file, format: {}, file: {}, chat: {:?}", format, file.file_id, cx.update.chat.kind);
    } else if res.status().is_client_error() {
        cx.answer(WRONG_INPUT_MSG).send().await?;
        debug!("Client request error, format: {}, file: {}, chat: {:?}", format, file.file_id, cx.update.chat.kind);
    } else if res.status().is_server_error() {
        cx.answer(SERVER_ERROR_MSG).send().await?;
        debug!("server error, format: {}, file: {}, chat: {:?}", format, file.file_id, cx.update.chat.kind);
    } else {
        cx.answer("Oops, some unknown error happened, please start over.").send().await?;
        warn!("unknown server response: {:?}, format: {}, file: {}, chat: {:?}", res, format, file.file_id, cx.update.chat.kind);
    };
    next(StartState)
}
