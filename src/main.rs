use iced::widget::{text, Container};
use iced::{executor, Length};
use iced::{Application, Command, Element, Settings, Subscription, Theme};
use chrono::prelude::*;
pub fn main() -> iced::Result {
    Clock::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Clock {
    now: DateTime<Local>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(DateTime<Local>),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                now: Local::now()
            
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Container::new(
            text(
                self.now
                    .time()
                    .to_string()
                    .split(".")
                    .collect::<Vec<&str>>()[0],
            )
            .size(200),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1)).map(|_| {
            Message::Tick(
                Local::now()
                   
            )
        })
    }
}
