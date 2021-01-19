use js_sys::Array;
use std::{cell::RefCell, convert::TryFrom};
use std::{rc::Rc, vec};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    MessageEvent, RtcConfiguration, RtcDataChannel, RtcDataChannelInit, RtcDataChannelState,
    RtcIceCandidateInit, RtcIceServer, RtcPeerConnection,
    RtcPeerConnectionIceEvent, RtcSessionDescription, RtcSessionDescriptionInit,
    RtcSignalingState, WebSocket,
};

use crate::utils::{
    participants::Participants,
    socket::{Candidate, SignalingMessage, SocketMessage},
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
            is_negotiating: false,
            candidates_buffer: vec![],
            signaling_channel_opened: false,
            data_channel: None,
        }
    }

    fn set_is_negotiating(&mut self, value: bool) {
        self.is_negotiating = value;
    }

    #[allow(unused_must_use)]
    pub fn connect(web_rtc: Rc<RefCell<WebRTC>>, from_to: Participants) {
        let ws = WebSocket::new("wss://glacial-beyond-33808.herokuapp.com").unwrap();

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

        let on_message_clone = web_rtc.clone();
        let on_message_callback = Closure::wrap(Box::new(move |message: MessageEvent| {
            let message = SocketMessage::try_from(message);
            match message {
                Ok(parsed) => WebRTC::handle_message(on_message_clone.clone(), parsed),
                Err(error) => log::error!("Oh No: {:?}", error),
            };
        }) as BoxDynMessageEvent);
        ws.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
        on_message_callback.forget();

        let cloned_ws = ws.clone();
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
                    let send_res = cloned_ws.send_with_str(json_from_client_message.as_ref());
                    if let Err(ex) = send_res {
                        log::error!("Could execute ice candidate callback {:?}", ex)
                    }
                }
            }) as BoxDynEvent<RtcPeerConnectionIceEvent>);

        let webrtc_signaling_clone = web_rtc.clone();
        let on_signaling_callback = Closure::wrap(Box::new(move |_: MessageEvent| {
            let mut borrowed_web_rtc = webrtc_signaling_clone.borrow_mut();
            let new_value =
                borrowed_web_rtc.connection.signaling_state() == RtcSignalingState::Stable;
            borrowed_web_rtc.set_is_negotiating(new_value);
            log::info!("signaling state change");
        }) as BoxDynMessageEvent);

        let webrtc_negotation_need_clone = web_rtc.clone();
        let more_clone = web_rtc.clone();

        let cloned_ws = ws.clone();
        let my_cloned_webrtc = web_rtc.clone();
        let send_sdp_callback = Closure::wrap(Box::new(move |_: JsValue| {
            let message_to_send = SocketMessage::SignalMessageFromClient {
                content: SignalingMessage::SDP {
                    message: my_cloned_webrtc
                        .borrow_mut()
                        .connection
                        .local_description()
                        .unwrap()
                        .sdp(),
                },
            };
            let message_to_send = serde_json::to_string(&message_to_send).unwrap();
            cloned_ws.send_with_str(&message_to_send);
        }) as BoxDynEvent<JsValue>);

        let negociation_success_callback = Closure::wrap(Box::new(move |descriptor: JsValue| {
            let description_init = RtcSessionDescriptionInit::try_from(descriptor).unwrap();
            more_clone
                .borrow_mut()
                .connection
                .set_local_description(&description_init)
                .then(&send_sdp_callback);
        }) as BoxDynEvent<JsValue>);

        let on_negociation_needed_callback = Closure::wrap(Box::new(move |event: JsValue| {
            let mut borrowed_web_rtc = webrtc_negotation_need_clone.borrow_mut();
            if !borrowed_web_rtc.is_negotiating {
                borrowed_web_rtc.is_negotiating = true;

                let print_error_callback =
                    Closure::wrap(Box::new(|err| log::error!("{:?}", err)) as BoxDynJsValue);
                borrowed_web_rtc
                    .connection
                    .create_offer()
                    .then(&negociation_success_callback)
                    .catch(&print_error_callback);

                // TODO: Do we need to forget ?
                // TODO no, we must live with our memories
                // send_sdp_callback.forget();
                // print_error_callback.forget();
                // negociation_success_callback.forget();
            }
        }) as BoxDynJsValue);

        let web_rtc_borrowed = web_rtc.borrow_mut();

        web_rtc_borrowed
            .connection
            .set_onicecandidate(Some(on_ice_candidate_callback.as_ref().unchecked_ref()));
        on_ice_candidate_callback.forget();

        web_rtc_borrowed
            .connection
            .set_onsignalingstatechange(Some(on_signaling_callback.as_ref().unchecked_ref()));
        on_signaling_callback.forget();

        web_rtc_borrowed.connection.set_onnegotiationneeded(Some(
            on_negociation_needed_callback.as_ref().unchecked_ref(),
        ));
        on_negociation_needed_callback.forget();
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
            SocketMessage::SignalMessageFromClient { content } => {}
        }
    }

    fn handle_user_here(web_rtc: Rc<RefCell<WebRTC>>, signaling_id: u16) {
        log::info!("Signaling message: {:?}", signaling_id);
        let cloned_web_rtc = web_rtc.clone();
        let mut web_rtc_borrow = cloned_web_rtc.as_ref().borrow_mut();
        if !web_rtc_borrow.signaling_channel_opened {
            let current_room = &web_rtc_borrow.room;
            // TODO: Mutability not required if we can chain method calls
            let mut data_channel_init = RtcDataChannelInit::new();
            data_channel_init.negotiated(true);
            data_channel_init.id(signaling_id);
            let data_channel = web_rtc_borrow
                .connection
                .create_data_channel_with_data_channel_dict(
                    &(current_room.as_ref().unwrap()),
                    &data_channel_init,
                );

            let on_message_data_channel_callback =
                Closure::wrap(Box::new(move |ev: MessageEvent| {
                    // TODO: Display this message as a YOU on the UI.
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

    #[allow(unused)]
    fn handle_ice_candidate(web_rtc: Rc<RefCell<WebRTC>>, candidate: Candidate) {
        let mut borrow_mut = web_rtc.borrow_mut();
        let peer_connection = &borrow_mut.connection;
        let remote_description: Option<RtcSessionDescription> =
            peer_connection.remote_description();
        if remote_description.is_none() {
            let candidate_init = RtcIceCandidateInit::new(&candidate.candidate);
            (borrow_mut).candidates_buffer.push(candidate_init);
        } else {
            let candidate_init = RtcIceCandidateInit::new(&candidate.candidate);
            let print_error_callback =
                Closure::wrap(Box::new(|err| log::error!("{:?}", err)) as BoxDynJsValue);
            let print_success_callback =
                Closure::wrap(Box::new(|success| log::error!("{:?}", success)) as BoxDynJsValue);

            peer_connection
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

        // TODO - TTO: refactor
        let send_sdp_callback = Closure::wrap(Box::new(move |_: JsValue| {
            let borrow_mut = clone.borrow_mut();
            let message_to_send = SocketMessage::SignalMessageFromClient {
                content: SignalingMessage::SDP {
                    message: borrow_mut.connection.local_description().unwrap().sdp(),
                },
            };
            let message_to_send = serde_json::to_string(&message_to_send).unwrap();
            // TODO: Set the socket in WebRTC object to keep the socket alive
            // cloned_ws.send_with_str(&message_to_send);
        }) as BoxDynEvent<JsValue>);

        let remote_description_success_callback = Closure::wrap(Box::new(|_: JsValue| {
            // if borrow_mut.connection.remote_description().unwrap().type_() == RtcSdpType::Offer {
            // borrow_mut
            //     .connection
            //     .create_answer()
            //     .then(&send_sdp_callback);
            // };
        }) as BoxDynEvent<JsValue>);
        let print_error_callback =
            Closure::wrap(Box::new(|err| log::error!("{:?}", err)) as BoxDynJsValue);

        let clone_2 = web_rtc.clone();
        let borrow_mut_2 = clone_2.borrow_mut();
        borrow_mut_2
            .connection
            .set_remote_description(&description_init)
            .then(&remote_description_success_callback)
            .catch(&print_error_callback);
    }
}

// function onSignalingMessageSDP(message) {
//     const {sdp} = JSON.parse(message);
//     rtcPeerConn.setRemoteDescription(sdp).then(() => {
//         // if we received an offer, we need to answer
//         if (rtcPeerConn.remoteDescription.type === 'offer') {
//             rtcPeerConn.createAnswer(sendLocalDesc, logError);
//         }
//         sendQueuedCandidates();
//     }).catch(logError);
// }

// function sendQueuedCandidates() {
//     candidatesQueue.forEach(candidate => {
//         rtcPeerConn.addIceCandidate(new RTCIceCandidate(candidate)).catch(err => console.error('error!!', err));
//     });
// }
