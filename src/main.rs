use iced::{Sandbox, Element, widget::TextInput};

pub struct HelloWorld{
    text: String,
    previous_inputs: Vec<String>
}


#[derive(Debug, Clone)]
pub enum Message {
    TextInputChanged(String),
    Submit
}

impl Sandbox for HelloWorld {
    type Message = Message;

    fn new() -> HelloWorld {
        HelloWorld { text: "".to_owned(), previous_inputs: Vec::new() }
    }

    fn title(&self) -> String {
        String::from("Hello, World!")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextInputChanged(string) => self.text = string,
            Message::Submit => self.previous_inputs.push(self.text.clone())
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let column = iced::widget::Column::new();
        let text_input = TextInput::new("Hello, World!", self.text.as_ref())
            .on_input(Message::TextInputChanged)
            .on_submit(Message::Submit);

        let button = iced::widget::button("submit")
            .on_press(Message::Submit);

        let row = iced::widget::Row::new();

        let row = row.push(text_input).push(button).padding(10).spacing(10);


        let column = column.push(row);

        let mut elements: Vec<_> = Vec::new();
        for element in &self.previous_inputs {
            elements.push(Element::from(iced::widget::Text::new(element.clone())));
        }
        let elements = iced::widget::Column::with_children(elements).padding(10);


        column
            .push(elements)
            .into()
    }
}

fn main() {
    let _ = HelloWorld::run(iced::Settings::default());
}
