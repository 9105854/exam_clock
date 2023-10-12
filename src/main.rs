use iced::widget::{button, column, container, row, text, text_input, Column};
use iced::{executor, Length, Renderer};
use iced::{theme, Alignment};
use iced::{Application, Command, Element, Settings, Subscription};
use std::num::ParseIntError;
pub fn main() -> iced::Result {
    println!("Developed by Dominik Beveridge. ");
    println!("Contact: drmbeveridge@gmail.com, or GitHub code repository");
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
    id: u32,
}
impl NewExam {
    fn new(clock: &Clock) -> Self {
        NewExam {
            name: "".to_string(),
            length: "".to_string(),
            perusal: "".to_string(),
            id: clock.exam_next_id,
        }
    }
}

struct StartedExam {
    name: String,
    perusal_start_time: chrono::DateTime<chrono::Local>,
    exam_start_time: chrono::DateTime<chrono::Local>,
    finish_time: chrono::DateTime<chrono::Local>,
    id: u32,
}

impl StartedExam {
    fn new_from_input(new_exam: NewExam) -> Result<Self, ParseIntError> {
        let perusal_start_time = chrono::Local::now() + chrono::Duration::minutes(1);
        let exam_start_time =
            perusal_start_time + chrono::Duration::minutes(new_exam.perusal.parse::<i64>()?);
        let finish_time =
            exam_start_time + chrono::Duration::minutes(new_exam.length.parse::<i64>()?);
        let name = if new_exam.name == "".to_string() {
            "Unnamed".to_string()
        } else {
            new_exam.name
        };
        Ok(StartedExam {
            name,
            perusal_start_time,
            exam_start_time,
            finish_time,
            id: new_exam.id,
        })
    }
}
struct Clock {
    now: chrono::DateTime<chrono::Local>,
    started_exams: Vec<StartedExam>,
    new_exams: Vec<NewExam>,
    theme: theme::Theme,
    exam_next_id: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    AddExam,
    InputExamMessage(InputExamMessage),
    DeleteExam(u32),
}
#[derive(Debug, Clone)]
enum InputExamMessage {
    NameEdit((String, u32)),

    LengthEdit((String, u32)),

    PerusalEdit((String, u32)),
    Start(u32),
}
enum TimeAccuracy {
    Seconds,
    Minutes,
}

fn format_time(time: chrono::DateTime<chrono::Local>, accuracy: TimeAccuracy) -> String {
    let binding = time.time().to_string();

    let formatted_time = &binding.split(":").collect::<Vec<&str>>();
    let hours = (formatted_time[0].parse::<i32>().unwrap() % 12).to_string();
    let minutes = formatted_time[1];
    match accuracy {
        TimeAccuracy::Seconds => {
            let seconds = formatted_time[2].split(".").collect::<Vec<&str>>()[0];

            format!("{}:{}:{}", hours, minutes, seconds)
        }
        TimeAccuracy::Minutes => {
            format!("{}:{}", hours, minutes)
        }
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
                new_exams: vec![],
                started_exams: vec![],
                theme: theme::Theme::Dark,
                exam_next_id: 2,
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
            Message::AddExam => {
                self.new_exams.push(NewExam::new(self));
                self.exam_next_id += 1;
            }
            Message::DeleteExam(id) => {
                let mut to_be_removed = 0;
                for (i, exam) in self.started_exams.iter().enumerate() {
                    if exam.id == id {
                        to_be_removed = i;
                        break;
                    }
                }
                self.started_exams.remove(to_be_removed);
            }
            Message::InputExamMessage(exam_message) => match exam_message {
                InputExamMessage::NameEdit((name, id)) => {
                    for exam in self.new_exams.iter_mut() {
                        if exam.id == id {
                            exam.name = name;
                            break;
                        }
                    }
                }
                InputExamMessage::LengthEdit((length, id)) => {
                    for exam in self.new_exams.iter_mut() {
                        if exam.id == id {
                            exam.length = length;
                            break;
                        }
                    }
                }
                InputExamMessage::PerusalEdit((perusal, id)) => {
                    for exam in self.new_exams.iter_mut() {
                        if exam.id == id {
                            exam.perusal = perusal;
                            break;
                        }
                    }
                }
                InputExamMessage::Start(id) => {
                    let mut to_be_removed = None;
                    for (i, exam) in self.new_exams.iter().enumerate() {
                        if exam.id == id {
                            let started_exam = StartedExam::new_from_input(exam.clone());
                            if let Ok(success) = started_exam {
                                self.started_exams.push(success);
                                to_be_removed = Some(i);
                            }
                            break;
                        }
                    }
                    if let Some(i) = to_be_removed {
                        self.new_exams.remove(i);
                    }
                }
            },
        }

        Command::none()
    }
    fn view(&self) -> Element<Message> {
        // exam_column.push(row![
        //  text(self.exams[0].name.clone()).size(50),
        // text(format!("Length: {length_string} minutes")).size(50),
        // ]);
        let new_exams: Column<'_, Message, Renderer> = column(
            self.new_exams
                .iter()
                .map(|new_exam| {
                    row![
                        text_input("Exam Name", &new_exam.name).on_input(|name| {
                            Message::InputExamMessage(InputExamMessage::NameEdit((
                                name.to_string(),
                                new_exam.id,
                            )))
                        }),
                        text_input("Perusal", &new_exam.perusal).on_input(|perusal| {
                            Message::InputExamMessage(InputExamMessage::PerusalEdit((
                                perusal.to_string(),
                                new_exam.id,
                            )))
                        }),
                        text_input("Length", &new_exam.length).on_input(|length| {
                            Message::InputExamMessage(InputExamMessage::LengthEdit((
                                length.to_string(),
                                new_exam.id,
                            )))
                        }),
                        button("Start Exam").on_press(Message::InputExamMessage(
                            InputExamMessage::Start(new_exam.id)
                        ))
                    ]
                    .spacing(10)
                    .into()
                })
                .collect(),
        )
        .spacing(10);
        let exams: Column<'_, Message, Renderer> = column(
            self.started_exams
                .iter()
                .map(|exam| {
                    container(
                        row![
                            text(format!("{}:", &exam.name))
                                .font(iced::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..iced::Font::default()
                                })
                                .size(50),
                            row![
                                text(format!(
                                    "Perusal: {}",
                                    format_time(exam.perusal_start_time, TimeAccuracy::Minutes)
                                ))
                                .size(50),
                                text(format!(
                                    "Start: {}",
                                    format_time(exam.exam_start_time, TimeAccuracy::Minutes)
                                ))
                                .size(50),
                                text(format!(
                                    "Finish: {}",
                                    format_time(exam.finish_time, TimeAccuracy::Minutes)
                                ))
                                .size(50),
                                button("Delete")
                                    .on_press(Message::DeleteExam(exam.id))
                                    .style(theme::Button::Destructive)
                            ]
                            .align_items(Alignment::Center)
                            .spacing(80)
                        ]
                        .align_items(Alignment::Center)
                        .spacing(80),
                    )
                    .height(100)
                    .center_y()
                    .center_x()
                    .into()
                })
                .collect(),
        )
        .spacing(10);
        column![
            container(text(format_time(self.now, TimeAccuracy::Seconds)).size(300))
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .center_y()
                .center_x(),
            column![
                container(exams).center_x().width(Length::Fill),
                container(new_exams).center_x().width(Length::Fill),
                container(button("Add new exam").on_press(Message::AddExam))
                    .width(Length::Fill)
                    .center_x()
            ]
            .spacing(30)
            .align_items(Alignment::Center)
            .height(Length::FillPortion(1))
            .width(Length::Fixed(1500.0))
        ]
        .align_items(Alignment::Center)
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
