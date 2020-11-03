use js_sys::Array;
use std::rc::Rc;
use std::{cell::RefCell, convert::TryFrom};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    MessageEvent, RtcConfiguration, RtcIceServer, RtcPeerConnection, RtcSessionDescriptionInit,
    WebSocket,
};

use crate::utils::{FromTo, SocketMessage};

type BoxDynJsValue = Box<dyn FnMut(JsValue)>;
type BoxDynMessageEvent = Box<dyn FnMut(MessageEvent)>;

pub struct WebRTC {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcPeerConnection.html
    pub connection: RtcPeerConnection,
    // This will act as a lifetime container for our callbacks.
    callbacks: Vec<Closure<dyn FnMut(JsValue)>>,
}

impl WebRTC {
    pub fn new() -> Self {
        let mut ice_server = RtcIceServer::new();
        ice_server.urls(&JsValue::from_str("stun:stun.services.mozilla.com"));

        let mut configuration = RtcConfiguration::new();
        configuration.ice_servers(&Array::of1(&ice_server));
        // TODO : Handle exception
        let peer_connection =
            RtcPeerConnection::new_with_configuration(&configuration).expect("OUPS");
        Self {
            connection: peer_connection,
            callbacks: vec![],
        }
    }

    #[allow(unused_must_use)]
    pub fn connect(web_rtc: Rc<RefCell<WebRTC>>, from_to: FromTo) {
        let ws = WebSocket::new("wss://glacial-beyond-33808.herokuapp.com").unwrap();

        // Is equivalent to onConnect in JS
        let _ = ws.clone();
        let cloned_ws = ws.clone();
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
                Ok(parsed) => log::info!("I parsed it correctly: {:?}", parsed),
                Err(error) => log::error!("Oh No: {:?}", error),
            }
        }) as BoxDynMessageEvent);
        ws.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
        on_message_callback.forget();

        // TODO : Handle exception
        // let _disney_channel = web_rtc
        //     .borrow_mut()
        //     .connection
        //     .create_data_channel("disney_channel");
        // web_rtc.borrow_mut().connection.peer_identity().then(&closure);

        // TODO : 1 Exchanging session descriptions
        //  Create an offer with a SDP
        // let web_rtc_manager_rc_clone = Rc::clone(&web_rtc);
        // let offer_function: BoxDynJsValue = Box::new(move |offer: JsValue| {
        //     // TODO: Error handling : dyn_into seems to not be recognized at runtime.
        //     let offer = offer.unchecked_into::<RtcSessionDescriptionInit>();

        //     log::info!("{:?}", offer);
        //     // TODO : Add catch handler closure
        //     web_rtc_manager_rc_clone
        //         .borrow()
        //         .connection
        //         .set_local_description(&offer);
        // });
        // let offer_callback = Closure::wrap(offer_function);

        // let exception_function: BoxDynJsValue = Box::new(|a: JsValue| {
        //     log::error!("An error occured during offer creation");
        //     log::error!("{:?}", &a);
        // });
        // let exception_callback = Closure::wrap(exception_function);

        // let _create_offer_promise = web_rtc
        //     .borrow_mut()
        //     .connection
        //     .create_offer()
        //     .then(&offer_callback)
        //     .catch(&exception_callback);

        // // Doing this ties the lifetime of the callback to the lifetime of the WebRtc object
        // web_rtc.borrow_mut().callbacks.push(offer_callback);
        // web_rtc.borrow_mut().callbacks.push(exception_callback);

        // TODO 2:  Exchanging ICE candidates

        // TODO 3: Listen SDP offers and send SDP answers
    }
}
