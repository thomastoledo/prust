#![recursion_limit = "1024"]

use std::{cell::RefCell, rc::Rc};

use yew::{Bridge, Component, ComponentLink, html, Html, ShouldRender};
use yew::agent::Bridged;

use components::chat_message::{ChatMessage, SenderType};
use event_bus::EventBus;
use utils::participants::Participants;
use web_rtc::WebRTC;

mod components;
mod utils;
mod web_rtc;
mod event_bus;

pub struct App {
    link: ComponentLink<Self>,
    chat_messages: Vec<ChatMessage>,
    web_rtc: Rc<RefCell<WebRTC>>,
    _producer: Box<dyn Bridge<EventBus>>,
}

pub enum ActionMessage {
    OnConnect(Participants),
    HandleMessage(ChatMessage),
}

impl Component for App {
    type Message = ActionMessage;
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let web_rtc_manager = Rc::new(RefCell::new(web_rtc::WebRTC::new()));
        let cloned_link = link.clone();
        Self {
            link,
            chat_messages: vec![],
            web_rtc: web_rtc_manager,
            _producer: EventBus::bridge(cloned_link.callback(ActionMessage::HandleMessage)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ActionMessage::HandleMessage(chat_message) => {
                if let SenderType::ME = chat_message.from {
                    web_rtc::WebRTC::send_webrtc_message(self.web_rtc.clone(), &chat_message.content);
                }
                self.chat_messages.push(chat_message);
            }
            ActionMessage::OnConnect(from_to) => {
                web_rtc::WebRTC::connect(self.web_rtc.clone(), from_to)
            }
        };
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="app">
                    <section class="app__connect">
                        <components::connect::Connect on_connect=self.link.callback(|fromTo: Participants| ActionMessage::OnConnect(fromTo))></components::connect::Connect>
                    </section>
                    <section class="app__chat">
                        <section class="conversation-container">
                            { self.chat_messages.iter().map(|message| message.view()).collect::<Html>() }
                        </section>
                        <components::chatbox::ChatBox/>
                    </section>
                </section>
            </>
        }
    }
}
