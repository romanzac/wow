mod wow_error;

use serde_derive::{Deserialize, Serialize};
use std::str;
use wow_error::WowError;

#[derive(Deserialize, Serialize, Debug)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Wow {
    pub quotes: Vec<Quote>,
}

#[derive(Hash, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct ProtoMsg {
    pub msg: String,
    pub data: String,
}

const PROTO_MESSAGES: [&str; 8] = [
    "quote-request",
    "challenge-response",
    "hash-request",
    "quote-response",
    "invalid-hash-format-error",
    "invalid-hash-error",
    "bad-message-error",
    "repeated-quote-request-error",
];

// Parse incoming message into ProtoMsg format
pub fn parse_msg_in(data: &[u8], size: usize) -> Result<ProtoMsg, WowError> {
    let msg_in = str::from_utf8(&data[..size]).unwrap();

    match serde_json::from_str::<ProtoMsg>(msg_in) {
        Ok(m) => {
            // Check if we received a valid/known message
            if PROTO_MESSAGES.contains(&m.msg.as_ref()) {
                Ok(m)
            } else {
                Err(WowError::InvalidProtocolMessage)
            }
        }
        Err(e) => Err(WowError::Parse(e)),
    }
}

// Prepare message to be sent
pub fn prepare_msg_out(msg: &str, data: &str) -> (ProtoMsg, Vec<u8>) {
    let msg_out = ProtoMsg {
        msg: msg.to_string(),
        data: data.to_string(),
    };
    let json_out = serde_json::to_string(&msg_out).unwrap();

    (msg_out, json_out.into_bytes())
}
