use web_sys::HtmlInputElement;
use yew::prelude::*;


pub struct Connect {
    link: ComponentLink<Self>,
    my_name: NodeRef,
    other_name: NodeRef,
}

pub enum Msg {
    ClickConnect(MouseEvent),
}

impl Component for Connect {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            my_name: NodeRef::default(),
            other_name: NodeRef::default()
        }
    }
        

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickConnect(e) => {
                e.prevent_default();
                if let Some(input_name) = self.my_name.cast::<HtmlInputElement>() {
                    log::debug!("MyName: {}", input_name.value())  
                };
                if let Some(other_name) = self.other_name.cast::<HtmlInputElement>() {
                    log::debug!("MyName: {}", other_name.value())  
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
                <form>
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



