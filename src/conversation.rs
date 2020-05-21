use yew::prelude::*;

pub struct Conversation {
    props: ConversationProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ConversationProps {
    pub chat_messages: Vec<String>,
}

impl Component for Conversation {
    type Message = ();
    type Properties = ConversationProps;

    // https://doc.rust-lang.org/rust-by-example/trait.html
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="conversation">
                    { self.props.chat_messages.iter().map(|message| format!("<p>{}</p>", message)).collect::<Html>() }
                </section>
            </>
        }
    }
}
