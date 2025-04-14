use iced::{Application, Settings, window, Color, Element, executor, Theme, Command, Subscription};
use iced::widget::{Container, Column, Text};

fn main() -> iced::Result {
    MatrixTerminal::run(Settings {
        window: window::Settings {
            size: (800, 600),
            min_size: Some((400, 300)),
            position: window::Position::Centered,
            title: String::from("Matrix Terminal"),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct MatrixTerminal;

#[derive(Debug, Clone)]
enum Message {
    // No messages yet
}

impl Application for MatrixTerminal {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Matrix Terminal")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Matrix Terminal")
                .size(30)
                .style(iced::theme::Text::Color(MATRIX_GREEN)));

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(MatrixStyle)))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

// Matrix colors
const MATRIX_GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.255,
    a: 1.0,
};

struct MatrixStyle;

impl iced::widget::container::StyleSheet for MatrixStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            text_color: Some(MATRIX_GREEN),
            background: Some(iced::Background::Color(Color::BLACK)),
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: MATRIX_GREEN,
        }
    }
}