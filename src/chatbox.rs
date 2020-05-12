use yew::prelude::*;
// use wasm_logger;

// #[cfg(feature = "web_sys")]
use web_sys::HtmlInputElement;

pub struct ChatBox {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    // props: ChatBoxProps,
}

#[derive(Properties, Clone)]
pub struct ChatBoxProps {
    pub onSend: Callback<String>,
}

pub enum Msg {
    SendMessage,
}

impl Component for ChatBox {
    type Message = Msg;
    type Properties = ();

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Self { link, node_ref: NodeRef::default(), props }
        Self { link, node_ref: NodeRef::default()}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendMessage => match self.node_ref.cast::<HtmlInputElement>() {
                Some(input) => println!("TOTO"), // log::info!("Text = {:?}", input.raw_value()),
                None => println!("TUTU"), // log::info!("No value !"),
            }
        };
        // In update
        // let has_attributes = self.node_ref.try_into::<Element>().has_attributes();
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
                <form class="chatbox__form">
                    <textarea ref=self.node_ref.clone() placeholder="Type something..."></textarea>
                    <button type="button" onclick=self.link.callback(|_| Msg::SendMessage)> {"Send"} </button>
                </form>
            </>
        }
    }
}
