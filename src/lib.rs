use chat_message::{ChatMessage, SenderType};
use yew::prelude::*;
mod chat_message;
mod chatbox;
mod connect_box;
mod web_rtc;
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    link: ComponentLink<Self>,
    chat_messages: Vec<ChatMessage>,
    web_rtc: Rc<RefCell<web_rtc::WebRTC>>,
    display_connect: bool,
}

pub enum ActionMessage {
    OnReceive(String),
    OnConnect(),
}

impl Component for App {
    type Message = ActionMessage;
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let web_rtc_manager = Rc::new(RefCell::new(web_rtc::WebRTC::new()));
        Self {
            link,
            chat_messages: vec![],
            web_rtc: web_rtc_manager.clone(),
            display_connect: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ActionMessage::OnReceive(value) => {
                let received_message = ChatMessage::new(SenderType::ME, value);
                self.chat_messages.push(received_message.clone());
                received_message.network_send();
            }
            ActionMessage::OnConnect() => {
                self.display_connect = !self.display_connect;
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
        let maybe_display_connect = move || -> Html {
            if self.display_connect {
                html! {
                  <connect_box::ConnectBox web_rtc=Rc::clone(&self.web_rtc)></connect_box::ConnectBox>
                }
            } else {
                html! {}
            }
        };
        html! {
            <>
                <section class="app">
                    <section class="conversation-container">
                        { self.chat_messages.iter().map(|message| message.view()).collect::<Html>() }
                    </section>
                    <chatbox::ChatBox
                        on_send=self.link.callback(|message: String| ActionMessage::OnReceive(message))
                        on_connect=self.link.callback(|_| ActionMessage::OnConnect())>
                    </chatbox::ChatBox>
                </section>
                {{maybe_display_connect()}}
            </>
        }
    }
}
