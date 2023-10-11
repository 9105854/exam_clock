use iced::widget::{button, column, container, row, text, text_input, Column};
use iced::{alignment, executor, Length, Renderer};
use iced::{theme, Alignment};
use iced::{Application, Command, Element, Settings, Subscription};
pub fn main() -> iced::Result {
    println!("Hello World");
    Clock::run(Settings {
        antialiasing: true,

        ..Settings::default()
    })
}
#[derive(Clone, Debug)]
struct NewExam {
    name: String,
    length: String,
    perusal: String,
}

#[derive(Clone, Debug)]
pub enum NewExamMessage {
    Started,
    LengthEdited(String),
}
impl NewExam {
    fn new() -> Self {
        NewExam {
            name: "".to_string(),
            length: "".to_string(),
            perusal: "".to_string(),
        }
    }
    fn update(&mut self, message: NewExamMessage) {
        match message {
            NewExamMessage::LengthEdited(new_length) => {
                self.length = new_length;
            }
            NewExamMessage::Started => {}
        }
    }

    fn view(&self) -> Element<NewExamMessage> {
        row![
            text_input("Exam Name", &self.name),
            text_input("Perusal Length", &self.perusal),
            text_input("Exam Length", &self.length).on_input(NewExamMessage::LengthEdited),
            button("Start Exam").on_press(NewExamMessage::Started)
        ]
        .into()
    }
}

struct StartedExam {
    name: String,
    perusal_start_time: chrono::DateTime<chrono::Local>,
    exam_start_time: chrono::DateTime<chrono::Local>,
    finish_time: chrono::DateTime<chrono::Local>,
}
struct Clock {
    now: chrono::DateTime<chrono::Local>,
    started_exams: Vec<StartedExam>,
    open_exams: Vec<NewExam>,
    theme: theme::Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    StartExam,
    OpenExamMessage(NewExamMessage),
}

fn format_time(time: chrono::DateTime<chrono::Local>) -> String {
    let binding = time.time().to_string();

    let formatted_time = &binding.split(":").collect::<Vec<&str>>();

    if formatted_time.len() >= 2 {
        format!("{}:{}", formatted_time[0], formatted_time[1])
    } else {
        format!("{}:{}", formatted_time[0], formatted_time[0])
    }
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                now: chrono::Local::now(),
                open_exams: vec![NewExam {
                    name: "Physics".to_string(),
                    length: String::from("120"),
                    perusal: String::from("10"),
                }],
                started_exams: vec![StartedExam {
                    name: "Maths".to_string(),
                    perusal_start_time: chrono::Local::now(),
                    exam_start_time: chrono::Local::now() + chrono::Duration::minutes(10),
                    finish_time: chrono::Local::now() + chrono::Duration::minutes(120),
                }],
                theme: theme::Theme::Dark,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                }
            }
            Message::StartExam => {}
            Message::OpenExamMessage(message) => match message {
                NewExamMessage::Started => {}
                NewExamMessage::LengthEdited(length) => {}
            },
        }

        Command::none()
    }
    fn view(&self) -> Element<Message> {
        // exam_column.push(row![
        //  text(self.exams[0].name.clone()).size(50),
        // text(format!("Length: {length_string} minutes")).size(50),
        // ]);
        let exams: Column<'_, Message, Renderer> = column(
            self.started_exams
                .iter()
                .enumerate()
                .map(|(_i, exam)| {
                    container(
                        row![
                            text(format!("{}:", &exam.name)).size(50),
                            row![
                                text(format!(
                                    "Start Perusal: {}",
                                    format_time(exam.perusal_start_time)
                                ))
                                .size(50),
                                text(format!("Start Exam: {}", format_time(exam.exam_start_time)))
                                    .size(50),
                                text(format!("Finish Exam: {}", format_time(exam.finish_time)))
                                    .size(50)
                            ]
                            .align_items(Alignment::Center)
                            .spacing(80)
                        ]
                        .align_items(Alignment::Center)
                        .spacing(80),
                    )
                    .height(100)
                    .center_y()
                    .into()
                })
                .collect(),
        )
        .spacing(30);
        column![
            container(
                text(
                    self.now
                        .time()
                        .to_string()
                        .split(".")
                        .collect::<Vec<&str>>()[0],
                )
                .size(300),
            )
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .center_y()
            .center_x(),
            container(exams)
                .center_x()
                .height(Length::FillPortion(1))
                .width(Length::Fill),
            self.open_exams[0]
                .view()
                .map(move |message| { Message::OpenExamMessage(message) }),
            button("Add new exam").on_press(Message::StartExam)
        ]
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1))
            .map(|_| Message::Tick(chrono::Local::now()))
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
