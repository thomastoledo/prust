use js_sys::Array;
use std::rc::Rc;
use std::{cell::RefCell, convert::TryFrom};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    MessageEvent, RtcConfiguration, RtcDataChannel, RtcDataChannelInit, RtcDataChannelState,
    RtcIceServer, RtcPeerConnection, RtcSessionDescriptionInit, WebSocket,
};

use crate::utils::{
    participants::Participants,
    socket::{SignalingMessage, SocketMessage},
};

type BoxDynJsValue = Box<dyn FnMut(JsValue)>;
type BoxDynMessageEvent = Box<dyn FnMut(MessageEvent)>;

pub struct WebRTC {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcPeerConnection.html
    pub connection: RtcPeerConnection,
    room: Option<String>,
    signaling_channel_opened: bool,
    data_channel: Option<RtcDataChannel>,
    // This will act as a lifetime container for our callbacks.
    callbacks: Vec<Closure<dyn FnMut(JsValue)>>,
}

impl WebRTC {
    pub fn new() -> Self {
        let mut ice_server = RtcIceServer::new();
        ice_server.urls(&JsValue::from_str("stun:stun.services.mozilla.com"));

        let mut configuration = RtcConfiguration::new();
        configuration.ice_servers(&Array::of1(&ice_server));
        let peer_connection = RtcPeerConnection::new_with_configuration(&configuration)
            .expect("Cannot create a Peer Connection");
        Self {
            connection: peer_connection,
            room: None,
            signaling_channel_opened: false,
            data_channel: None,
            callbacks: vec![],
        }
    }

    #[allow(unused_must_use)]
    pub fn connect(web_rtc: Rc<RefCell<WebRTC>>, from_to: Participants) {
        let ws = WebSocket::new("wss://glacial-beyond-33808.herokuapp.com").unwrap();

        // let _ = ws.clone();
        let cloned_ws = ws.clone();
        // Is equivalent to onConnect in JS
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            log::info!("socket opened");

            let new_user_message = SocketMessage::NewUser {
                content: from_to.clone(),
            };
            let json_new_user_message = serde_json::to_string(&new_user_message).unwrap();

            let send_res = cloned_ws.send_with_str(json_new_user_message.as_ref());
            match send_res {
                Ok(_) => (),
                Err(ex) => log::error!("Could not connect to websocket {:?}", ex),
            }
        }) as BoxDynJsValue);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let onclose_callback = Closure::wrap(Box::new(move |_| {
            log::info!("socket closed");
        }) as BoxDynJsValue);
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        let on_message_callback = Closure::wrap(Box::new(move |message: MessageEvent| {
            let message = SocketMessage::try_from(message);
            match message {
                Ok(parsed) => WebRTC::handle_message(web_rtc.clone(), parsed),
                Err(error) => log::error!("Oh No: {:?}", error),
            };
        }) as BoxDynMessageEvent);
        ws.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
        on_message_callback.forget();
    }

    pub fn send_message(web_rtc: Rc<RefCell<WebRTC>>, message: &str) {
        if let Some(data_channel) = &(web_rtc.as_ref().borrow()).data_channel {
            if data_channel.ready_state() == RtcDataChannelState::Open {
                data_channel
                    .send_with_str(message)
                    .expect("THIS WILL WORK !");
            }
        };
    }

    fn handle_message(web_rtc: Rc<RefCell<WebRTC>>, socket_message: SocketMessage) {
        match socket_message {
            SocketMessage::JoinedRoom { content } => {
                log::info!("JoinedRoom message: {:?}", &content.room);
                // TODO: This panic on borrow error
                (*web_rtc.borrow_mut()).room = Some(content.room);
            }
            SocketMessage::NewUser { .. } => {}
            SocketMessage::SignalMessageToClient {
                content: SignalingMessage::UserHere { message },
            } => {
                log::info!("Signaling message: {:?}", message);
                let cloned_web_rtc = web_rtc.clone();
                let mut web_rtc_borrow = cloned_web_rtc.as_ref().borrow_mut();
                if !web_rtc_borrow.signaling_channel_opened {
                    let current_room = &web_rtc_borrow.room;
                    // TODO: Mutability not required if we can chain method calls
                    let mut data_channel_init = RtcDataChannelInit::new();
                    data_channel_init.negotiated(true);
                    data_channel_init.id(message);
                    let data_channel = web_rtc_borrow
                        .connection
                        .create_data_channel_with_data_channel_dict(
                            &(current_room.as_ref().unwrap()),
                            &data_channel_init,
                        );

                    let on_message_data_channel_callback =
                        Closure::wrap(Box::new(move |ev: MessageEvent| {
                            if let Some(message) = ev.data().as_string() {
                                log::warn!("{:?}", message);
                            } else {
                                log::warn!("NOPE");
                            }
                        }) as BoxDynMessageEvent);

                    data_channel.set_onmessage(Some(
                        on_message_data_channel_callback.as_ref().unchecked_ref(),
                    ));
                    on_message_data_channel_callback.forget();
                    web_rtc_borrow.data_channel = Some(data_channel);
                }
            }
            SocketMessage::SignalMessageToClient {
                content: SignalingMessage::ICECandidate { message },
            } => {
                log::info!("ICECandidate message: {:?}", message);
            }
            SocketMessage::SignalMessageToClient {
                content: SignalingMessage::SDP { message },
            } => {
                log::info!("SDP message: {:?}", message);
            }
        }
    }
}
