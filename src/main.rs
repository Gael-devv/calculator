use iced::{
    Sandbox, Settings, Element, Length, Theme, 
    widget, alignment, window::Settings as WindowSettings
};
use std::fmt::{self, Display};

fn main() -> iced::Result {
    Calculator::run(Settings {
        window: WindowSettings {
            size: (400, 610),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Calculator {
    num: String,
    result: f32,
    op: Option<Operation>
}

#[derive(Debug, Clone)]
enum Message {
    C,
    Ce,
    Num(String),
    Add(&'static str),
    Del,
    Op(Operation),
    Negative,
    Result
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum,
    Sub,
    Mul,
    Div,
}

impl Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operation::Sum => "+",
            Operation::Sub => "-",
            Operation::Mul => "x",
            Operation::Div => "/",
        };

        write!(f, "{symbol}")
    }
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Calculator { num: String::new(), result: 0.0, op: None }
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::C => {
                self.result = 0.0;
                self.num.clear();
            }
            Message::Ce => {
                self.num.clear();
            }
            Message::Num(n) => {
                if n.trim().parse::<f32>().is_ok() | n.is_empty() {
                    self.num = n
                }
            },
            Message::Add(n) => {
                self.num.push_str(n);
            }
            Message::Del => {
                self.num.pop();
            },
            Message::Op(op) => {
                self.op = Some(op);
                self.result = self.num.trim().parse().unwrap_or(0.0);
                self.num.clear();
            }
            Message::Negative => {
                self.num.insert_str(0, "-");
            },
            Message::Result => {
                let num = self.num.trim().parse::<f32>().unwrap_or(0.0);
                match self.op {
                    Some(Operation::Sum) => self.result += num,
                    Some(Operation::Sub) => self.result -= num,
                    Some(Operation::Mul) => self.result *= num,
                    Some(Operation::Div) => self.result /= num,
                    None => return,
                }

                self.num.clear();
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let btn = |sym, msg| {
            widget::button(
                widget::text(sym)
                    .size(64)
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .height(Length::Units(100))
            .width(Length::Units(100))
            .on_press(msg)
        };

        let num_btn = |num: &'static str| {
            btn(num.to_string(), Message::Add(num))
        };

        let op_btn = |op: Operation| {
            btn(op.to_string(), Message::Op(op))
        };

        let input = widget::text_input(
            &self.result.to_string(), 
            &self.num, 
            Message::Num
        ).size(100);

        widget::column![
            input,
            widget::row![
                btn("C".to_string(), Message::C),
                btn("CE".to_string(), Message::Ce),
                btn("DEL".to_string(), Message::Del),
                op_btn(Operation::Div)
            ],
            widget::row![
                num_btn("1"),
                num_btn("2"),
                num_btn("3"),
                op_btn(Operation::Mul)
            ],
            widget::row![
                num_btn("4"),
                num_btn("5"),
                num_btn("6"),
                op_btn(Operation::Sub)
            ],
            widget::row![
                num_btn("7"),
                num_btn("8"),
                num_btn("9"),
                op_btn(Operation::Sum)
            ],
            widget::row![
                btn("±".to_string(), Message::Negative),
                num_btn("0"),
                num_btn("."),
                btn("=".to_string(), Message::Result)
            ],
        ].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
