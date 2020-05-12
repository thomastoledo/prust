use yew::prelude::*;
// use wasm_logger;

mod chatbox;
mod conversation;

struct AppPrust {}

impl Component for AppPrust {
    type Message = ();
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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
                <chatbox::ChatBox></chatbox::ChatBox>
            </>
        }
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::default());

    // Logging
    // log::info!("Starting");
    yew::initialize();
    App::<AppPrust>::new().mount_to_body();
    // log::info!("Started");
}
