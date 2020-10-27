use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SocketMessage {
    #[serde(rename = "newUser")]
    NewUser { content: FromTo },
    #[serde(rename = "signal_message_to_client")]
    SignalMessageToClient { content: SignalingMessage },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "signalType")]
pub enum SignalingMessage {
    #[serde(rename = "userHere")]
    UserHere {message: u16}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FromTo {
    #[serde(rename = "userFrom")]
    pub user_from: String,
    #[serde(rename = "userTo")]
    pub user_to: String,
}