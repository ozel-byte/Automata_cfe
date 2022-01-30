use iced::{
    text_input::TextInput, Button, Column, Container, Element, HorizontalAlignment, Length,
    Row, Sandbox, Text,
};

use super::{automata, button_next, style, text_input_enter};

#[derive(Default)]
pub struct AutomataCfe {
    theme: style::style::Theme,
    input: text_input_enter::TextInputEnter,
    button_validation: button_next::ButtonNext,
    service_number: String,
    date: String,
    price: String,
    validation_digit: String,
    number: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    DataInputChanged(String),
    ButtonValidationDigit,
}

impl Sandbox for AutomataCfe {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("CFE-201235-201245")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DataInputChanged(mut data) => {
                data.truncate(30);
                self.input.input_value = data;
            }
            Message::ButtonValidationDigit => {
                let mut run_automata = automata::automata::ValidarCFE {
                    valid: String::from("0"),
                    text_entry: self.input.input_value.clone(),
                    slide_entry: self.input.input_value.clone(),
                    date_paid: String::new(),
                    amount_paid: String::new(),
                    service_number: String::new(),
                };

                run_automata.estado_0();
                //numero completo
                //Numero de servicio
                //Fecha
                //Importe
                self.service_number = run_automata.service_number;
                self.price = run_automata.amount_paid;
                self.date = run_automata.date_paid;
                self.number = run_automata.text_entry;
                self.validation_digit = run_automata.valid;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let column_component_input_button: Column<_> = Column::new()
            .push(
                TextInput::new(
                    &mut self.input.input,
                    "Type the code",
                    &self.input.input_value,
                    Message::DataInputChanged,
                )
                .width(Length::Units(300))
                .padding(10),
            )
            .push(
                Button::new(
                    &mut self.button_validation.btn,
                    Text::new("Check")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(iced::VerticalAlignment::Center),
                )
                .on_press(Message::ButtonValidationDigit)
                .width(Length::Units(250))
                .style(self.theme)
                .height(Length::Units(35)),
            )
            .align_items(iced::Align::Center)
            .spacing(20);

        let column_content = if self.validation_digit != "2" {
            Column::new()
                .push(
                    Text::new("Numero de recibo").horizontal_alignment(HorizontalAlignment::Center),
                )
                .push(Text::new(&self.number).horizontal_alignment(HorizontalAlignment::Center))
                .push(Text::new("Numero de servicio"))
                .push(Text::new(&self.service_number))
                .push(Text::new("Fecha"))
                .push(Text::new(&self.date))
                .push(Text::new("Importe"))
                .push(Text::new(&self.price))
                .spacing(20)
        } else {
            Column::new().push(Text::new("Error de codigo"))
        };

        let row_component_li: Row<_> = Row::new()
            .push(column_component_input_button)
            .push(column_content)
            .spacing(30);

        Container::new(row_component_li)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()

        //Text::new(&self.number).width(Length::Fill)
    }
}
