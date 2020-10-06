use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::prelude::*;
use yew::Callback;

pub struct ChatBox {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    props: ChatBoxProps,
}

impl ChatBox {
    fn send_message(&self) {
        if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
            self.props.on_send.emit(input.value());
            input.set_value("");
        }
    }
}

#[derive(Properties, Clone)]
pub struct ChatBoxProps {
    pub on_send: Callback<String>,
}

pub enum Msg {
    SendMessage,
    ReturnCarriage(KeyboardEvent),
}

impl Component for ChatBox {
    type Message = Msg;
    type Properties = ChatBoxProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            node_ref: NodeRef::default(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendMessage => self.send_message(),

            Msg::ReturnCarriage(e) => {
                if e.key_code() == 13 && !e.ctrl_key() && !e.shift_key() {
                    self.send_message();
                    e.prevent_default();
                }
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
                    <textarea
                        ref=self.node_ref.clone()
                        onkeydown=self.link.callback(|e: KeyboardEvent| Msg::ReturnCarriage(e))
                        placeholder="Type something...">
                    </textarea>
                    <input type="button" onclick=self.link.callback(|_| Msg::SendMessage) class="material-icons" value="flight_takeoff"/>
                </form>
                
            </>
        }
    }
}
