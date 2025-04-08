#![windows_subsystem = "windows"]
use iced::{
    Font, Settings, Size,
    widget::{Column, button, column, text},
    window,
};

fn main() -> iced::Result {
    iced::application("DEMO", Counter::update, Counter::view)
        .settings(Settings {
            default_font: Font::with_name("微软雅黑"),
            ..Settings::default()
        })
        .window(window::Settings {
            size: Size {
                width: 400.0,
                height: 300.0,
            },
            ..window::Settings::default()
        })
        .run()
}

#[derive(Debug)]
struct Counter {
    value: Vec<bool>,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Update,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            value: vec![false; 5],
            index: 0,
        }
    }
}

impl Counter {
    fn set_true_at_index(&mut self) {
        if self.index == self.value.len() {
            self.index = 0;
        }
        for i in 0..self.value.len() {
            match i == self.index {
                true => self.value[i] = true,
                false => self.value[i] = false,
            }
        }
        self.index += 1;
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Update => self.set_true_at_index(),
        }
    }

    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            text(format!("{:?}", self.value)),
            // We show the value of the counter here
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("update").on_press(Message::Update),
        ]
    }
}
