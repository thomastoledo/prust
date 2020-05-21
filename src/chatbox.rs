use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Callback;

pub struct ChatBox {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    props: ChatBoxProps,
}

#[derive(Properties, Clone)]
pub struct ChatBoxProps {
    pub on_send: Callback<String>,
}

pub enum Msg {
    SendMessage,
}

impl Component for ChatBox {
    type Message = Msg;
    type Properties = ChatBoxProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, node_ref: NodeRef::default(), props}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendMessage => match self.node_ref.cast::<HtmlInputElement>() {
                Some(input) => self.props.on_send.emit(input.value()),
                None => log::error!("No value !"),
            }
        };
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
                    <input type="button" onclick=self.link.callback(|_| Msg::SendMessage) value="Send"/>
                </form>
            </>
        }
    }
}
