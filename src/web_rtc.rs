use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::{RtcConfiguration, RtcIceServer, RtcPeerConnection};

pub struct WebRTC {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcPeerConnection.html
    connection: RtcPeerConnection,
    // This will act as a lifetime container for our callbacks.
    callbacks: Vec<Closure<dyn FnMut(JsValue)>>
}

impl WebRTC {
    pub fn new() -> Self {
        let mut ice_server = RtcIceServer::new();
        ice_server.urls(&JsValue::from_str("stun:stun.services.mozilla.com"));

        let mut configuration = RtcConfiguration::new();
        configuration.ice_servers(&Array::of1(&ice_server));
        Self {
            // TODO : Handle errors
            connection: RtcPeerConnection::new_with_configuration(&configuration).expect("OUPS"),
            callbacks: vec!()
        }
    }

    pub fn connect(&mut self) {
        let disney_channel = self.connection.create_data_channel("disney_channel");
        // connection.peer_identity().then(&closure);

        // TODO : 1 Exchanging session descriptions
        //  Create an offer with a SDP
        let exception_callback = Closure::wrap(Box::new(move |a: JsValue| {
            log::error!("An error occured during offer creation");
            log::error!("{:?}", &a);
        }) as Box<dyn FnMut(JsValue)>);
        let callback = Closure::wrap(Box::new(|offer: JsValue| {
            log::info!("{:?}", offer);
        }) as Box<dyn FnMut(JsValue)>);
        self.connection
            .create_offer()
            .then(&callback)
            .catch(&exception_callback);

        // We could do this but this is a memory leak.
        // callback.forget();

        // Doing this ties the lifetime of the callback to the lifetime of the WebRtc object
        self.callbacks.push(callback);
        self.callbacks.push(exception_callback);

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
