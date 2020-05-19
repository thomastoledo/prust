use yew::prelude::*;

mod chatbox;
mod conversation;

struct Callbacks {
    on_msg: Callback<String>
}

struct AppPrust {
    // callbacks: Callbacks,
}

enum TestMessage { 
    OnReceive
}

impl Component for AppPrust {
    type Message = TestMessage;
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let callbacks: Callbacks = {
        //     on_msg: link.send_message(msg: T)(|value: String| log::info!("{}", value))
        // };
        // Self {callbacks}
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <conversation::Conversation></conversation::Conversation>
                // <chatbox::ChatBox on_send=|_| TestMessage::OnReceive ></chatbox::ChatBox>
            </>
        }
    }
}

// |value: String| log::error!("{}", value)

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<AppPrust>();
}
