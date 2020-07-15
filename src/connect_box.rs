use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

use crate::web_rtc;

pub struct ConnectBox {
    link: ComponentLink<Self>,
    connect_type: ConnectType,
    props: ConnectProps,
}

enum ConnectType {
    Client,
    Server,
    Undefined,
}

pub enum ConnectMessages {
    ShowClient,
    ShowServer,
    Connect,
}

#[derive(Properties, Clone)]
pub struct ConnectProps {
    pub web_rtc: Rc<RefCell<web_rtc::WebRTC>>,
}

impl Component for ConnectBox {
    type Message = ConnectMessages;
    type Properties = ConnectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            connect_type: ConnectType::Undefined,
            props,
        }
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            ConnectMessages::ShowClient => self.connect_type = ConnectType::Client,
            ConnectMessages::ShowServer => {
                self.connect_type = ConnectType::Server;
                web_rtc::WebRTC::connect(Rc::clone(&self.props.web_rtc));
            }
            ConnectMessages::Connect => self.connect_type = ConnectType::Undefined,
        };
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let display_from_type = || -> Html {
            match self.connect_type {
                ConnectType::Undefined => html! {
                    <>
                        <button onclick=self.link.callback(|_| ConnectMessages::ShowServer)>{{"As server"}}</button>
                        <button onclick=self.link.callback(|_| ConnectMessages::ShowClient)>{{"As client"}}</button>
                    </>
                },
                ConnectType::Client => html! {
                    <>
                        <textarea class="connect_box__textarea"></textarea>
                        <button onclick=self.link.callback(|_| ConnectMessages::Connect)>{{"Connect"}}</button>
                    </>
                },
                ConnectType::Server => html! {
                    <>
                        <p>{format!("{:?}", self.props.web_rtc.borrow().connection.local_description())}</p>
                        <button onclick=self.link.callback(|_| ConnectMessages::Connect)>{{"Connect"}}</button>
                    </>
                },
            }
        };

        html! {
            <>
              <section class="connect_box">
                {{display_from_type()}}
              </section>
            </>
        }
    }
}
