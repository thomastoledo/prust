use std::fmt::{Display, Debug, Formatter, Result};
use js_sys::Date;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SenderType {
    ME,
    YOU,
}

impl Display for SenderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SenderType::ME => write!(f, "me"),
            SenderType::YOU => write!(f, "u"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    from: SenderType,
    content: String,
    pub timestamp: f64,
}

impl ChatMessage {
    pub fn new(from: SenderType, content: String) -> Self {
        Self { from, content, timestamp: Date:: now() }
    }

    pub fn view(&self) -> Html {
        html! {
            <p class=format!("message--{}", self.from)>{html! { &self.content.replace("\n", "<br/>")} }</p>
        }
    }
}
