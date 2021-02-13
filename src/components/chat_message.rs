use std::fmt::{Debug, Display, Formatter, Result};

use serde::{Deserialize, Serialize};
use yew::prelude::*;

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
    pub from: SenderType,
    pub content: String,
}

impl ChatMessage {
    pub fn new(from: SenderType, content: String) -> Self {
        Self { from, content }
    }

    pub fn view(&self) -> Html {
        html! {
            <p class=format!("message--{}", self.from)>{html! { &self.content.replace("\n", "<br/>")} }</p>
        }
    }
}
