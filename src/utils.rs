use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use web_sys::MessageEvent;

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// TODO: This is a mess !!!
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SocketMessage {
    #[serde(rename = "newUser")]
    NewUser { content: FromTo },
    #[serde(rename = "signal_message_to_client")]
    SignalMessageToClient { content: SignalingMessage },
}

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

impl TryFrom<MessageEvent> for SocketMessage {
    type Error = CustomError;

    fn try_from(message_event: MessageEvent) -> Result<Self, Self::Error> {
        let string_data = message_event
            .data()
            .as_string()
            .ok_or(CustomError::InputTypeError(String::from(
                "MessageEvent.data is not a String",
            )))?;
            // serde_json::from_str::<SocketMessage>(&string_data)
        let res = serde_json::from_str::<SocketMessage>(&string_data)?;
        Ok(res)
        // TODO: I would like to finish with l51 :sad_me:
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "signalType")]
pub enum SignalingMessage {
    #[serde(rename = "userHere")]
    UserHere { message: u16 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FromTo {
    #[serde(rename = "userFrom")]
    pub user_from: String,
    #[serde(rename = "userTo")]
    pub user_to: String,
}
