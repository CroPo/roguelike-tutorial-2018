use tcod::Color;
use tcod::colors;
use std::cell::RefCell;
use std::cell::Ref;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub color: Color,
}

impl Message {
    pub fn new(text: String, color: Color) -> Message {
        Message {
            text,
            color,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MessageLog {
    messages: RefCell<Vec<Message>>
}

impl MessageLog {
    pub fn new() -> MessageLog {
        MessageLog {
            messages: RefCell::new(vec![])
        }
    }

    pub fn add(&self, message: Message) {
        self.messages.borrow_mut().push(message);
    }

    pub fn messages(&self) -> Ref<Vec<Message>> {
        self.messages.borrow()
    }
}

