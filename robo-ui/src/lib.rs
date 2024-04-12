use iced::widget::{column, container, row, slider, text};
use iced::{alignment, executor, Alignment, Application, Element, Length, Theme};

pub struct Controls {
    base: u8,
    shoulder: u8,
    elbow: u8,
    wrist: u8,
    rotation: u8,
    limits: Limits,
}

struct Limits {
    base: (u8, u8),
    shoulder: (u8, u8),
    elbow: (u8, u8),
    wrist: (u8, u8),
    rotation: (u8, u8),
}

#[derive(Debug, Clone)]
pub enum Joint {
    Base(u8),
    Shoulder(u8),
    Elbow(u8),
    Wrist(u8),
    Rotation(u8),
}

const SPACE: u16 = 10;

macro_rules! slider_block {
    ($left:expr, $middle:expr, $right:expr) => {
        row![
            $left
                .horizontal_alignment(alignment::Horizontal::Right)
                .width(Length::Fill),
            $middle.width(Length::FillPortion(5)),
            $right
                .horizontal_alignment(alignment::Horizontal::Left)
                .width(Length::Fill),
        ]
        .spacing(SPACE)
    };
}

impl Application for Controls {
    type Message = Joint;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        "Robotic Arm".into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Controls {
                base: 50,
                shoulder: 50,
                elbow: 50,
                wrist: 50,
                rotation: 50,
                limits: Limits {
                    base: (0, 100),
                    shoulder: (0, 100),
                    elbow: (0, 100),
                    wrist: (0, 100),
                    rotation: (0, 100),
                },
            },
            iced::Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Self::Message::Base(value) => {
                self.base = value;
            }
            Self::Message::Shoulder(value) => {
                self.shoulder = value;
            }
            Self::Message::Elbow(value) => {
                self.elbow = value;
            }
            Self::Message::Wrist(value) => {
                self.wrist = value;
            }
            Self::Message::Rotation(value) => {
                self.rotation = value;
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let bl = self.limits.base;
        let base_controls = column![slider_block![
            text(format!("{}", bl.0)),
            slider(bl.0..=bl.1, self.base, Self::Message::Base),
            text(format!("{}", bl.1))
        ]]
        .align_items(Alignment::Center)
        .spacing(SPACE);

        let sl = self.limits.shoulder;
        let shoulder_controls = column![slider_block![
            text(format!("{}", sl.0)),
            slider(sl.0..=sl.1, self.shoulder, Self::Message::Shoulder),
            text(format!("{}", sl.1))
        ]]
        .align_items(Alignment::Center)
        .spacing(SPACE);

        let el = self.limits.elbow;
        let elbow_controls = column![slider_block![
            text(format!("{}", el.0)),
            slider(el.0..=el.1, self.elbow, Self::Message::Elbow),
            text(format!("{}", el.1))
        ]]
        .align_items(Alignment::Center)
        .spacing(SPACE);

        let wl = self.limits.wrist;
        let wrist_controls = column![slider_block![
            text(format!("{}", wl.0)),
            slider(wl.0..=wl.1, self.wrist, Self::Message::Wrist),
            text(format!("{}", wl.1))
        ]]
        .align_items(Alignment::Center)
        .spacing(SPACE);

        let rl = self.limits.rotation;
        let rotation_controls = column![slider_block![
            text(format!("{}", rl.0)),
            slider(rl.0..=rl.1, self.rotation, Self::Message::Rotation),
            text(format!("{}", rl.1))
        ]]
        .align_items(Alignment::Center)
        .spacing(SPACE);

        let content: Element<_> = column![
            base_controls,
            shoulder_controls,
            elbow_controls,
            wrist_controls,
            rotation_controls,
        ]
        .align_items(Alignment::Center)
        .spacing(SPACE)
        .padding(SPACE)
        .into();

        container(content).center_x().into()
    }
}
