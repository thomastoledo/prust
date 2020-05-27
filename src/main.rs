use yew::prelude::*;

mod chatbox;
mod conversation;

struct AppPrust {
    link: ComponentLink<Self>, 
    chat_messages: Vec<String>,
}

enum TestMessage { 
    OnReceive(String),
}

impl Component for AppPrust {
    type Message = TestMessage;
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {link, chat_messages: vec![String::from("Good morning Vietnam !")]}
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
                <conversation::Conversation chat_messages=&self.chat_messages></conversation::Conversation>
                <chatbox::ChatBox on_send=self.link.callback(|message: String| TestMessage::OnReceive(message))>
                </chatbox::ChatBox>
            </section>
        }
    }
}

// |value: String| log::error!("{}", value)

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<AppPrust>();
}
