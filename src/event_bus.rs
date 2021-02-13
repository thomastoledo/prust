use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use yew::worker::*;

use crate::components::chat_message::ChatMessage;

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    EventBusMsg(ChatMessage),
}

impl Agent for EventBus {
    type Reach = Context;
    type Message = ();
    type Input = Request;
    type Output = ChatMessage;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::EventBusMsg(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
                }
            }
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
