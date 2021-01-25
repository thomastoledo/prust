use js_sys::Array;
use std::{cell::RefCell, convert::TryFrom};
use std::{rc::Rc, vec};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    MessageEvent, RtcConfiguration, RtcDataChannel, RtcDataChannelInit, RtcDataChannelState,
    RtcIceCandidateInit, RtcIceServer, RtcPeerConnection, RtcPeerConnectionIceEvent, RtcSdpType,
    RtcSessionDescription, RtcSessionDescriptionInit, RtcSignalingState, WebSocket,
};

use crate::utils::{
    participants::Participants,
    socket::{Candidate, Room, SignalingMessage, SocketMessage},
};

type BoxDynJsValue = Box<dyn FnMut(JsValue)>;
type BoxDynMessageEvent = Box<dyn FnMut(MessageEvent)>;
type BoxDynEvent<T> = Box<dyn FnMut(T)>;

pub struct WebRTC {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcPeerConnection.html
    pub connection: RtcPeerConnection,
    room: Option<String>,
    signaling_channel_opened: bool,
    is_negotiating: bool,
    candidates_buffer: Vec<RtcIceCandidateInit>,
    data_channel: Option<RtcDataChannel>,
    socket: WebSocket,
}

impl WebRTC {
    pub fn new() -> Self {
        let mut ice_server = RtcIceServer::new();
        ice_server.urls(&JsValue::from_str("stun:stun.services.mozilla.com"));

        let mut configuration = RtcConfiguration::new();
        configuration.ice_servers(&Array::of1(&ice_server));
        let peer_connection = RtcPeerConnection::new_with_configuration(&configuration)
            .expect("Cannot create a Peer Connection");

        let socket = WebSocket::new("wss://glacial-beyond-33808.herokuapp.com").unwrap();

        // Is equivalent to onConnect in JS
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            log::info!("socket opened");
        }) as BoxDynJsValue);
        socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let onclose_callback = Closure::wrap(Box::new(move |_| {
            log::info!("socket closed");
        }) as BoxDynJsValue);
        socket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        Self {
            connection: peer_connection,
            room: None,
            is_negotiating: false,
            candidates_buffer: vec![],
            signaling_channel_opened: false,
            data_channel: None,
            socket,
        }
    }

    fn set_is_negotiating(&mut self, value: bool) {
        self.is_negotiating = value;
    }

    pub fn connect(web_rtc: Rc<RefCell<WebRTC>>, participants: Participants) {
        let on_message_clone = web_rtc.clone();
        let on_message_callback = Closure::wrap(Box::new(move |message: MessageEvent| {
            let message = SocketMessage::try_from(message);
            match message {
                Ok(parsed) => WebRTC::handle_message(on_message_clone.clone(), parsed),
                Err(error) => log::error!("Oh No: {:?}", error),
            };
        }) as BoxDynMessageEvent);
        {
            // To avoid multiple borrow in the same time we borrow in this limited scope
            web_rtc
                .clone()
                .as_ref()
                .borrow()
                .socket
                .set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
            on_message_callback.forget();
        }

        let on_ice_cloned = web_rtc.clone();
        let on_ice_candidate_callback =
            Closure::wrap(Box::new(move |event: RtcPeerConnectionIceEvent| {
                log::info!("on ice candidate callback");
                if event.candidate().is_some() {
                    let signal_message_from_client = SocketMessage::SignalMessageFromClient {
                        content: SignalingMessage::ICECandidate {
                            message: Candidate {
                                candidate: event.candidate().unwrap().candidate(),
                            },
                        },
                    };

                    let json_from_client_message =
                        serde_json::to_string(&signal_message_from_client).unwrap();
                    let send_res = on_ice_cloned
                        .as_ref()
                        .borrow()
                        .socket
                        .send_with_str(json_from_client_message.as_ref());
                    if let Err(ex) = send_res {
                        log::error!("Could not execute ice candidate callback {:?}", ex)
                    }
                }
            }) as BoxDynEvent<RtcPeerConnectionIceEvent>);
        let webrtc_signaling_clone = web_rtc.clone();
        let on_signaling_callback = Closure::wrap(Box::new(move |_: MessageEvent| {
            let new_value = webrtc_signaling_clone
                .as_ref()
                .borrow()
                .connection
                .signaling_state()
                == RtcSignalingState::Stable;
            webrtc_signaling_clone
                .as_ref()
                .borrow_mut()
                .set_is_negotiating(new_value);
            log::info!("signaling state change");
        }) as BoxDynMessageEvent);

        let sdp_clone = web_rtc.clone();
        let send_sdp_callback = Closure::wrap(Box::new(move |_: JsValue| {
            let message_to_send = SocketMessage::SignalMessageFromClient {
                content: SignalingMessage::SDP {
                    message: sdp_clone
                        .as_ref()
                        .borrow()
                        .connection
                        .local_description()
                        .unwrap()
                        .sdp(),
                },
            };
            let message_to_send = serde_json::to_string(&message_to_send).unwrap();
            match sdp_clone.as_ref().borrow().socket.send_with_str(&message_to_send) {
                Ok(_) => log::info!("Successfully handle sdp callback"),
                Err(err) => log::error!("Error in sdp callback {:?}", err),
            };
        }) as BoxDynEvent<JsValue>);

        let on_negociation_success_clone = web_rtc.clone();
        let negociation_success_callback = Closure::wrap(Box::new(move |descriptor: JsValue| {
            let description_init = RtcSessionDescriptionInit::try_from(descriptor).unwrap();
            let _ = on_negociation_success_clone
                .as_ref()
                .borrow()
                .connection
                .set_local_description(&description_init)
                .then(&send_sdp_callback);
        }) as BoxDynEvent<JsValue>);

        let on_negociation_needed_clone = web_rtc.clone();
        let on_negociation_needed_callback = Closure::wrap(Box::new(move |_: JsValue| {
            let mut borrow_mut = on_negociation_needed_clone.as_ref().borrow_mut();
            if !borrow_mut.is_negotiating {
                borrow_mut.set_is_negotiating(true);

                let print_error_callback =
                    Closure::wrap(Box::new(|err| log::error!("{:?}", err)) as BoxDynJsValue);
                let _ = borrow_mut
                    .connection
                    .create_offer()
                    .then(&negociation_success_callback)
                    .catch(&print_error_callback);
            }
        }) as BoxDynJsValue);
        {
            // Creates a scope to avoid multiple borrow mut.
            web_rtc
                .as_ref()
                .borrow()
                .connection
                .set_onicecandidate(Some(on_ice_candidate_callback.as_ref().unchecked_ref()));
            on_ice_candidate_callback.forget();

            web_rtc
                .as_ref()
                .borrow()
                .connection
                .set_onsignalingstatechange(Some(on_signaling_callback.as_ref().unchecked_ref()));
            on_signaling_callback.forget();

            web_rtc.as_ref().borrow().connection.set_onnegotiationneeded(Some(
                on_negociation_needed_callback.as_ref().unchecked_ref(),
            ));
            on_negociation_needed_callback.forget();

            // Send message in socket
            let new_user_message = SocketMessage::NewUser {
                content: participants.clone(),
            };
            let json_new_user_message = serde_json::to_string(&new_user_message).unwrap();
            let send_res = web_rtc
                .as_ref()
                .borrow()
                .socket
                .send_with_str(json_new_user_message.as_ref());
            match send_res {
                Ok(_) => (),
                Err(ex) => log::error!("Could not connect to websocket {:?}", ex),
            }
        }
    }

    pub fn send_message(web_rtc: Rc<RefCell<WebRTC>>, message: &str) {
        if let Some(data_channel) = &web_rtc.as_ref().borrow().data_channel {
            if data_channel.ready_state() == RtcDataChannelState::Open {
                match data_channel.send_with_str(message) {
                    Ok(_) => log::info!("Message sent"),
                    Err(err) => log::error!("Could not send message {:?}", err),
                }
            }
        };
    }

    fn handle_message(web_rtc: Rc<RefCell<WebRTC>>, socket_message: SocketMessage) {
        match socket_message {
            SocketMessage::JoinedRoom { content } => {
                WebRTC::join_room(web_rtc, content);
            }
            SocketMessage::NewUser { .. } => {}
            SocketMessage::SignalMessageToClient {
                content: SignalingMessage::UserHere { message },
            } => {
                WebRTC::handle_user_here(web_rtc, message);
            }
            SocketMessage::SignalMessageToClient {
                content: SignalingMessage::ICECandidate { message },
            } => {
                WebRTC::handle_ice_candidate(web_rtc, message);
            }
            SocketMessage::SignalMessageToClient {
                content: SignalingMessage::SDP { message },
            } => {
                WebRTC::handle_sdp_message(web_rtc, &message);
            }
            SocketMessage::SignalMessageFromClient { .. } => {}
        }
    }

    fn join_room(web_rtc: Rc<RefCell<WebRTC>>, content: Room) {
        log::info!("JoinedRoom message: {:?}", &content.room);
        (*web_rtc.as_ref().borrow_mut()).room = Some(content.room.clone());
    }

    fn handle_user_here(web_rtc: Rc<RefCell<WebRTC>>, signaling_id: u16) {
        log::info!("Signaling message: {:?}", signaling_id);
        let cloned_web_rtc = web_rtc.clone();
        let mut borrow_mut = cloned_web_rtc.as_ref().borrow_mut();
        if !borrow_mut.signaling_channel_opened {
            let current_room = &borrow_mut.room;
            // TODO: Mutability not required if we can chain method calls
            let mut data_channel_init = RtcDataChannelInit::new();
            data_channel_init.negotiated(true);
            data_channel_init.id(signaling_id);
            let data_channel = borrow_mut
                .connection
                .create_data_channel_with_data_channel_dict(
                    &(current_room.as_ref().unwrap()),
                    &data_channel_init,
                );

            let on_message_data_channel_callback =
                Closure::wrap(Box::new(move |ev: MessageEvent| {
                    // TODO: Display this message as a YOU on the UI.
                    if let Some(message) = ev.data().as_string() {
                        log::warn!("Received message {:?}", message);
                    } else {
                        log::warn!("Received message error");
                    }
                }) as BoxDynMessageEvent);

            data_channel.set_onmessage(Some(
                on_message_data_channel_callback.as_ref().unchecked_ref(),
            ));
            on_message_data_channel_callback.forget();
            borrow_mut.data_channel = Some(data_channel);
        }
    }

    fn handle_ice_candidate(web_rtc: Rc<RefCell<WebRTC>>, candidate: Candidate) {
        log::info!("Handle ice candidate start");
        let cloned_web_rtc = web_rtc.clone();

        let mut borrowed = cloned_web_rtc.as_ref().borrow_mut();
        let remote_description: Option<RtcSessionDescription> =
            borrowed.connection.remote_description();
        if remote_description.is_none() {
            log::info!("Remote description is none");
            let candidate_init = RtcIceCandidateInit::new(&candidate.candidate);
            borrowed.candidates_buffer.push(candidate_init);
            log::info!("Remote description is DONE");
        } else {
            log::info!("Remote description is some");
            let candidate_init = RtcIceCandidateInit::new(&candidate.candidate);
            let print_error_callback = Closure::wrap(Box::new(|err| {
                log::error!("remote description is some - error{:?}", err)
            }) as BoxDynJsValue);
            let print_success_callback = Closure::wrap(Box::new(|success| {
                log::info!("remote description is some - success{:?}", success)
            }) as BoxDynJsValue);

            let _ = borrowed
                .connection
                .add_ice_candidate_with_opt_rtc_ice_candidate_init(Some(&candidate_init))
                .then(&print_success_callback)
                .catch(&print_error_callback);
            print_error_callback.forget();
            print_success_callback.forget();
        }
    }

    fn handle_sdp_message(web_rtc: Rc<RefCell<WebRTC>>, sdp: &str) {
        let description_init = RtcSessionDescriptionInit::from(JsValue::from_str(sdp));
        let clone = web_rtc.clone();

        let send_sdp_callback = Closure::wrap(Box::new(move |_: JsValue| {
            let borrow_mut = clone.borrow_mut();
            let message_to_send = SocketMessage::SignalMessageFromClient {
                content: SignalingMessage::SDP {
                    message: borrow_mut.connection.local_description().unwrap().sdp(),
                },
            };
            let message_to_send = serde_json::to_string(&message_to_send).unwrap();
            // TODO: Set the socket in WebRTC object to keep the socket alive
            match borrow_mut.socket.send_with_str(&message_to_send) {
                    Ok(_) => log::info!("binary message successfully sent"),
                    Err(err) => log::error!("error sending message: {:?}", err),
            }
        }) as BoxDynEvent<JsValue>);

        let clone_remote_description_success = web_rtc.clone();
        let remote_description_success_callback = Closure::wrap(Box::new(move |_: JsValue| {
            if clone_remote_description_success
                .as_ref()
                .borrow()
                .connection
                .remote_description()
                .unwrap()
                .type_()
                == RtcSdpType::Offer
            {
                let _ = clone_remote_description_success
                        .as_ref()
                        .borrow()
                        .connection
                        .create_answer()
                        .then(&send_sdp_callback);
            }
            // send Queued Candidates
            for candidate in &clone_remote_description_success.as_ref().borrow().candidates_buffer {
                    let _ = clone_remote_description_success
                        .as_ref()
                        .borrow()
                        .connection
                        .add_ice_candidate_with_opt_rtc_ice_candidate_init(Some(&candidate));
                log::info!("candidate unqueuing success");
            }
        }) as BoxDynEvent<JsValue>);

        let clone_2 = web_rtc.clone();
        let _ = clone_2
            .as_ref()
            .borrow()
            .connection
            .set_remote_description(&description_init)
            .then(&remote_description_success_callback);
    }
}
