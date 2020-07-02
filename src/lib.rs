use chat_message::{ChatMessage, SenderType};
use yew::prelude::*;
mod chat_message;
mod chatbox;
mod web_rtc;
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    link: ComponentLink<Self>,
    chat_messages: Vec<ChatMessage>,
    web_rtc: Rc<RefCell<web_rtc::WebRTC>>,
}

pub enum ActionMessage {
    OnReceive(String),
}

impl Component for App {
    type Message = ActionMessage;
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let web_rtc_manager = Rc::new(RefCell::new(web_rtc::WebRTC::new()));
        web_rtc::WebRTC::connect(Rc::clone(&web_rtc_manager));
        Self {
            link,
            chat_messages: vec![],
            web_rtc: web_rtc_manager.clone(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ActionMessage::OnReceive(value) => {
                let received_message = ChatMessage::new(SenderType::ME, value);
                self.chat_messages.push(received_message.clone());
                received_message.network_send();
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
            <section class="app">
                <section class="conversation-container">
                    { self.chat_messages.iter().map(|message| message.view()).collect::<Html>() }
                </section>
                <chatbox::ChatBox on_send=self.link.callback(|message: String| ActionMessage::OnReceive(message))>
                </chatbox::ChatBox>
            </section>
        }
    }
}
