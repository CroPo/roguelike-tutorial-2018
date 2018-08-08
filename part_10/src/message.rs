use std::cell::RefCell;
use std::cell::Ref;

use json::JsonValue;

use tcod::Color;
use tcod::colors;

use savegame::{Serialize, Deserialize};

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

impl Serialize for Message {
    fn serialize(&self) -> JsonValue {

        let mut color = JsonValue::new_array();

        color.push(self.color.r);
        color.push(self.color.g);
        color.push(self.color.b);

        object!(
            "text" => self.text.clone(),
            "color" => color
        )
    }
}

impl Deserialize for Message {
    fn deserialize(json: &JsonValue) -> Self {

        Message {
            text: json["text"].as_str().unwrap().to_string(),
            color: Color {
                r : json["color"][0].as_u8().unwrap(),
                g : json["color"][1].as_u8().unwrap(),
                b : json["color"][2].as_u8().unwrap(),
            }
        }
    }
}

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

impl Serialize for MessageLog {
    fn serialize(&self) -> JsonValue {
        let mut messages = JsonValue::new_array();
        for message in self.messages.borrow().iter() {
            messages.push(message.serialize());
        }
        messages
    }
}

impl Deserialize for MessageLog {
    fn deserialize(json: &JsonValue) -> Self {
        let mut log = Self::new();

        for m in json.members() {
            log.add(Message::deserialize(m))
        }
        log
    }
}