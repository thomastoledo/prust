use std::convert::TryFrom;

use crate::Participants;
use serde::{Deserialize, Serialize};
use web_sys::MessageEvent;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SocketMessage {
    #[serde(rename = "newUser")]
    NewUser { content: Participants },
    #[serde(rename = "signal_message_to_client")]
    SignalMessageToClient { content: SignalingMessage },
    #[serde(rename = "joined_room")]
    JoinedRoom { content: Room },
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub room: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "signalType")]
pub enum SignalingMessage {
    #[serde(rename = "userHere")]
    UserHere { message: u16 },
    #[serde(rename = "ice_candidate")]
    ICECandidate {message : Candidate},
    #[serde(rename = "SDP")]
    SDP {message: String}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    candidate: String
}

impl TryFrom<MessageEvent> for SocketMessage {
    type Error = CustomError;

    fn try_from(message_event: MessageEvent) -> Result<Self, Self::Error> {
        let string_data = message_event.data().as_string().ok_or_else(|| {
            CustomError::InputTypeError(String::from("MessageEvent.data is not a String"))
        })?;
        Ok(serde_json::from_str::<SocketMessage>(&string_data)?)
    }
}

// ERRORS
#[derive(Debug)]
pub enum CustomError {
    InputTypeError(String),
    JsonParseError(serde_json::Error),
}

impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError::JsonParseError(error)
    }
}
