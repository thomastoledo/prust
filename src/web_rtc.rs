use js_sys::Array;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{RtcConfiguration, RtcIceServer, RtcPeerConnection, RtcSessionDescriptionInit};

pub struct WebRTC {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcPeerConnection.html
    connection: RtcPeerConnection,
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
        let peer_connection = RtcPeerConnection::new_with_configuration(&configuration).expect("OUPS"); 
        Self {
            connection: peer_connection,
            callbacks: vec![],
        }
    }

    #[allow(unused_must_use)]
    pub fn connect(web_rtc: Rc<RefCell<WebRTC>>) {
        // TODO : Handle exception
        let _disney_channel = web_rtc.borrow_mut().connection.create_data_channel("disney_channel");
        // web_rtc.borrow_mut().connection.peer_identity().then(&closure);

        // TODO : 1 Exchanging session descriptions
        //  Create an offer with a SDP
        let web_rtc_manager_rc_clone = Rc::clone(&web_rtc);
        let offer_function: Box<dyn FnMut(JsValue)> = Box::new(move |offer: JsValue| {
            // TODO: Error handling : dyn_into seems to not be recognized at runtime.
            let offer = offer.unchecked_into::<RtcSessionDescriptionInit>();

            // TODO : Add catch handler closure
            web_rtc_manager_rc_clone
            .borrow()
            .connection
            .set_local_description(&offer);
        });
        let offer_callback = Closure::wrap(offer_function);

        let exception_function: Box<dyn FnMut(JsValue)> = Box::new(|a: JsValue| {
            log::error!("An error occured during offer creation");
            log::error!("{:?}", &a);
        });
        let exception_callback = Closure::wrap(exception_function);

        let _create_offer_promise = web_rtc
            .borrow_mut()
            .connection
            .create_offer()
            .then(&offer_callback)
            .catch(&exception_callback);

        // We could do this but this is a memory leak.
        // callback.forget();

        // Doing this ties the lifetime of the callback to the lifetime of the WebRtc object
        web_rtc.borrow_mut().callbacks.push(offer_callback);
        web_rtc.borrow_mut().callbacks.push(exception_callback);

        // TODO 2:  Exchanging ICE candidates

        // TODO 3: Listen SDP offers and send SDP answers

        // let candidate = Some(&RtcIceCandidate::candidate("duwhqudhuwqdhuiwq hiudqhw uidhuwq hidhqwiu "))
        // connection.add_ice_candidate_with_opt_rtc_ice_candidate()
        // connection.set_onicecandidate(Some(|e| => )
    }
}

// function connectPeers() {
//     // Create the data channel and establish its event listeners
//     sendChannel = localConnection.createDataChannel("sendChannel");
//     sendChannel.onopen = handleSendChannelStatusChange;
//     sendChannel.onclose = handleSendChannelStatusChange;

//     // Create the remote connection and its event listeners

//     remoteConnection = new RTCPeerConnection();
//     remoteConnection.ondatachannel = receiveChannelCallback;

//     // Set up the ICE candidates for the two peers

//     localConnection.onicecandidate = e => !e.candidate
//         || remoteConnection.addIceCandidate(e.candidate)
//         .catch(handleAddCandidateError);

//     remoteConnection.onicecandidate = e => !e.candidate
//         || localConnection.addIceCandidate(e.candidate)
//         .catch(handleAddCandidateError);

//     // Now create an offer to connect; this starts the process

//     localConnection.createOffer()
//     .then(offer => localConnection.setLocalDescription(offer))
//     .then(() => remoteConnection.setRemoteDescription(localConnection.localDescription))
//     .then(() => remoteConnection.createAnswer())
//     .then(answer => remoteConnection.setLocalDescription(answer))
//     .then(() => localConnection.setRemoteDescription(remoteConnection.localDescription))
//     .catch(handleCreateDescriptionError);
//   }

// function receiveChannelCallback(event) {
//     receiveChannel = event.channel;
//     receiveChannel.onmessage = handleReceiveMessage;
//     receiveChannel.onopen = handleReceiveChannelStatusChange;
//     receiveChannel.onclose = handleReceiveChannelStatusChange;
//   }
