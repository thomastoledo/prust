use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Callback;

use crate::utils::FromTo;

pub struct Connect {
    link: ComponentLink<Self>,
    my_name: NodeRef,
    other_name: NodeRef,
    display_connect: bool,
    props: ConnectProps,
}

#[derive(Properties, Clone)]
pub struct ConnectProps {
    pub on_connect: Callback<FromTo>,
}

pub enum Msg {
    ClickConnect(MouseEvent),
}

impl Component for Connect {
    type Message = Msg;
    type Properties = ConnectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            my_name: NodeRef::default(),
            other_name: NodeRef::default(),
            props,
            display_connect: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickConnect(e) => {
                e.prevent_default();

                if let (Some(input_name), Some(other_name)) = (
                    self.my_name.cast::<HtmlInputElement>(),
                    self.other_name.cast::<HtmlInputElement>(),
                ) {
                    log::debug!("MyName: {}", input_name.value());
                    log::debug!("MyName: {}", other_name.value());
                    // Emit this in lib.rs
                    self.props
                        .on_connect
                        .emit(FromTo(input_name.value(), other_name.value()));
                    self.display_connect = false;
                } else {
                    log::error!("Both names are mandatory");
                };
            }
        };

        // TODO: call server for connexion between A & B
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
                <form class={format!("connect__form {}", if !self.display_connect {"connected"} else {""})}>
                    <label for="name">{"Your Name"}</label><br/>
                    <input id="name" ref=self.my_name.clone() type="text"/><br/><br/>

                    <label for="recipient">{"Your friend's name"}</label><br/>
                    <input id="recipient" ref=self.other_name.clone() type="text"/><br/><br/>

                    <button id="connect" onclick=self.link.callback(|e: MouseEvent| Msg::ClickConnect(e))>
                        {"Connect"}
                    </button>
                </form>
            </>
        }
    }
}
