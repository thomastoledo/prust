use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

mod chatbox;
mod conversation;

struct AppPrust {
    link: ComponentLink<Self>,
    chat_messages: Vec<String>,
    // children: ChildrenWithProps<ChatMessage>,
}

enum TestMessage {
    OnReceive(String),
}

impl Component for AppPrust {
    type Message = TestMessage;
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            chat_messages: vec![String::from("Good morning Vietnam !")],
            // children: ChildrenWithProps::new(vec![ChatMessage::create((), ())])
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TestMessage::OnReceive(value) => self.chat_messages.push(value),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="app">
                <section class="conversation-container">
                    { self.chat_messages.iter().map(|message| "<ChatMessage></ChatMessage>").collect::<Html>() }
                </section>
                // <conversation::Conversation chat_messages=&self.chat_messages></conversation::Conversation>
                <chatbox::ChatBox on_send=self.link.callback(|message: String| TestMessage::OnReceive(message))>
                </chatbox::ChatBox>
            </section>
        }
    }
}

enum SenderType {
    ME,
    YOU,
}

impl Display for SenderType { 
    
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SenderType::ME => write!(f, "me"),
            SenderType::YOU => write!(f, "u")
        }
    }
}

struct ChatMessage {
    from: SenderType,
    content: String,
}

impl Component for ChatMessage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            from: SenderType::ME,
            content: String::from("TOTO"),
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }
    
    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }
    
    fn view(&self) -> Html {
        html! {
            <span class=format!("message--{}", self.from)>
            </span>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<AppPrust>();
}
