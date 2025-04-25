use iced::highlighter::Theme;
use iced::widget::text::Wrapping;
use iced::widget::text_editor::{Action, Content};
use iced::widget::{checkbox, column, radio, row, text_editor, text_input};
use iced::Length::{Fill, FillPortion};
use iced::{Element, Font, Task};
use native_dialog::MessageLevel;

use crate::utils::{cell, label};

#[derive(Debug)]
pub struct FileSettings {
    pub modal: bool,
    pub title: String,
    pub location: String,
    pub filters: Content,
}

impl Default for FileSettings {
    fn default() -> Self {
        Self {
            modal: true,
            title: "Example File Dialog".to_string(),
            location: "~".to_string(),
            filters: Content::with_text(include_str!("filters.yml")),
        }
    }
}

#[derive(Debug)]
pub struct MsgSettings {
    pub level: MessageLevel,
    pub title: String,
    pub text: Content,
}

impl Default for MsgSettings {
    fn default() -> Self {
        Self {
            level: MessageLevel::Info,
            title: "Example Message Dialog".to_string(),
            text: Content::with_text("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit..."),
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub file: FileSettings,
    pub msg: MsgSettings,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileTitle(String),
    FileLocation(String),
    FileFilters(Action),
    FileModal(bool),
    MsgTitle(String),
    MsgText(Action),
    MsgLevel(MessageLevel),
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FileTitle(title) => state.file.title = title,
        Message::FileLocation(location) => state.file.location = location,
        Message::FileFilters(action) => state.file.filters.perform(action),
        Message::FileModal(modal) => state.file.modal = modal,
        Message::MsgTitle(title) => state.msg.title = title,
        Message::MsgText(action) => state.msg.text.perform(action),
        Message::MsgLevel(level) => state.msg.level = level,
    };

    Task::none()
}

pub fn view(state: &State) -> Element<Message> {
    column![
        cell(
            column![
                "File Dialog Settings",
                checkbox("Modal", state.file.modal).on_toggle(Message::FileModal),
                column![
                    label("Title"),
                    text_input("", &state.file.title).on_input(Message::FileTitle),
                ]
                .spacing(2),
                column![
                    label("Location"),
                    text_input("", &state.file.location).on_input(Message::FileLocation),
                ]
                .spacing(2),
                column![
                    label("Filters"),
                    text_editor(&state.file.filters)
                        .font(Font::MONOSPACE)
                        .size(14)
                        .wrapping(Wrapping::WordOrGlyph)
                        .highlight("yaml", Theme::InspiredGitHub)
                        .on_action(Message::FileFilters)
                        .height(Fill),
                ]
                .spacing(2),
            ]
            .spacing(8)
            .into()
        )
        .height(FillPortion(4)),
        cell(
            column![
                "Message Dialog Settings",
                row![
                    radio(
                        "Info",
                        MessageLevel::Info,
                        Some(state.msg.level),
                        Message::MsgLevel
                    ),
                    radio(
                        "Warning",
                        MessageLevel::Warning,
                        Some(state.msg.level),
                        Message::MsgLevel
                    ),
                    radio(
                        "Error",
                        MessageLevel::Error,
                        Some(state.msg.level),
                        Message::MsgLevel
                    ),
                ]
                .spacing(32),
                column![
                    label("Title"),
                    text_input("", &state.msg.title).on_input(Message::MsgTitle),
                ]
                .spacing(2),
                column![
                    label("Text"),
                    text_editor(&state.msg.text)
                        .wrapping(Wrapping::WordOrGlyph)
                        .on_action(Message::MsgText)
                        .height(Fill),
                ]
                .spacing(2),
            ]
            .spacing(8)
            .into()
        )
        .height(FillPortion(3)),
    ]
    .spacing(12)
    .into()
}
