use iced::{
    Alignment, 
    Row, 
    Text, 
    TextInput,
    Sandbox, 
    Element,
};

#[derive(Default)]
struct Temperature {
    c_value: f32,
    f_value: f32,
    c_text_state: iced::text_input::State,
    f_text_state: iced::text_input::State,
}

impl Sandbox for Temperature {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Temperature")
    }

    fn view(&mut self) -> Element<Message> {
        Row::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                TextInput::new(&mut self.c_text_state, "35", self.c_value.to_string().as_str(), Message::CInputChange).size(30)
            )
            .spacing(20)
            .push(
                Text::new("C to F")
            )
            .push(
                TextInput::new(&mut self.f_text_state, "35", self.f_value.to_string().as_str(), Message::FInputChange).size(30)
            )
            .into()
    }
    
    fn update(&mut self, message: Message) {
        match message {
            Message::FInputChange(string_val) => {
                self.f_value = match string_val.parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => 0.0,
                };
                self.c_value = (self.f_value - 32.0) * 5.0 / 9.0;
            },
            Message::CInputChange(string_val) => {
                self.c_value = match string_val.parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => 0.0,
                };
                self.f_value = (self.c_value * 9.0 / 5.0) + 32.0;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FInputChange(String),
    CInputChange(String),
}

fn main() -> Result<(), iced::Error> {
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = (250, 100);
    window_settings.position = iced::window::Position::Specific(0, 0);
    let mut settings = iced::Settings::default();
    settings.window = window_settings;
    Temperature::run(settings)
}
