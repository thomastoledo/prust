use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::{RtcConfiguration, RtcIceServer, RtcPeerConnection};
pub struct WebRTC {}

impl WebRTC {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcPeerConnection.html
    pub fn connect() {
        let mut ice_server = RtcIceServer::new();
        ice_server.urls(&JsValue::from_str("stun:stun.services.mozilla.com"));

        let mut configuration = RtcConfiguration::new();
        configuration.ice_servers(&Array::of1(&ice_server));

        // TODO : Handle errors
        let connection = RtcPeerConnection::new_with_configuration(&configuration).expect("OUPS");
        let disney_channel = connection.create_data_channel("disney_channel");
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
