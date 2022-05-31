use iced::{button, Row};
use iced::{Button, Error, Column, Text, Sandbox, Element, Alignment};

#[derive(Default)]
struct Counter {
    value: i32,
    count_button: button::State,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Test Counter")
    }

    fn view(&mut self) -> Element<Message> {
        Row::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Text::new(self.value.to_string()).size(50)
            )
            .spacing(20)
            .push(
                Button::new(&mut self.count_button, Text::new("Count"))
                    .on_press(Message::CountPressed)
            )
            .into()
    }
    
    fn update(&mut self, message: Message) {
        match message {
            Message::CountPressed => {
                self.value += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CountPressed,
}

fn main() -> Result<(), iced::Error> {
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = (150, 100);
    window_settings.position = iced::window::Position::Specific(0, 0);
    let mut settings = iced::Settings::default();
    settings.window = window_settings;
    Counter::run(settings)
}
