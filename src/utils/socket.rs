use std::{convert::TryFrom, fmt::Debug};

use crate::Participants;
use serde::{Deserialize, Serialize};
use web_sys::{MessageEvent, RtcSdpType, RtcSessionDescription, RtcSessionDescriptionInit};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SocketMessage {
    #[serde(rename = "newUser")]
    NewUser { content: Participants },
    #[serde(rename = "signal_message_from_client")]
    SignalMessageFromClient { content: SignalingMessage },
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
    ICECandidate { message: Candidate },
    #[serde(rename = "SDP")]
    SDP { message: SDPMessage },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SDPMessage {
    #[serde(rename = "type")]
    pub type_: String,
    pub sdp: String,
}

impl TryFrom<RtcSessionDescription> for SDPMessage {
    type Error = CustomError;

    fn try_from(value: RtcSessionDescription) -> Result<Self, Self::Error> {
        // TODO: This mapping looks fishy, improve this.
        let type_ = match value.type_() {
            RtcSdpType::Answer => "answer",
            RtcSdpType::Offer => "offer",
            RtcSdpType::Pranswer => "pranswer",
            RtcSdpType::Rollback => "rollback",
            _ => {
                return Result::Err(CustomError::InputTypeError(String::from(
                    "Unknown SDP type",
                )))
            }
        };
        Ok(SDPMessage {
            type_: String::from(type_),
            sdp: String::from(value.sdp()),
        })
    }
}

impl TryFrom<SDPMessage> for RtcSessionDescriptionInit {
    type Error = CustomError;

    fn try_from(value: SDPMessage) -> Result<Self, Self::Error> {
        // TODO: This mapping looks fishy, improve this.
        let type_ = match value.type_.as_str() {
            "answer" => RtcSdpType::Answer,
            "offer" => RtcSdpType::Offer,
            "pranswer" => RtcSdpType::Pranswer,
            "rollback" => RtcSdpType::Rollback,
            _ => {
                return Result::Err(CustomError::InputTypeError(String::from(
                    "SDP type not found",
                )));
            }
        };
        let mut res = RtcSessionDescriptionInit::new(type_);
        res.sdp(&value.sdp);
        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_m_line_index: u16,
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
