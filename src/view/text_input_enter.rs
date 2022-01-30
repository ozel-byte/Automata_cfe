use iced::{text_input};

#[derive(Default)]
pub struct TextInputEnter{
   pub input: text_input::State,
   pub input_value: String,
}
