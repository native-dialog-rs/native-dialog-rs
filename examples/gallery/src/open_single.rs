use std::path::PathBuf;

use iced::highlighter::Theme;
use iced::widget::text::Wrapping;
use iced::widget::text_editor::{Action, Content};
use iced::widget::{button, column, row, text_editor};
use iced::Length::Fill;
use iced::{Element, Font, Task};

use crate::settings::FileSettings;
use crate::utils::build_file_dialog;

#[derive(Debug, Default)]
pub struct State {
    output: Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    Show,
    Spawn,
    Update(Option<PathBuf>),
    Editor(Action),
}

pub fn update(state: &mut State, settings: &FileSettings, message: Message) -> Task<Message> {
    match message {
        Message::Show => show_dialog(settings),
        Message::Spawn => spawn_dialog(settings),
        Message::Update(output) => {
            state.output = Content::with_text(&format!("{:?}", output));
            Task::none()
        }
        Message::Editor(action) => {
            if !action.is_edit() {
                state.output.perform(action);
            }
            Task::none()
        }
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    column![
        "Open Single File",
        text_editor(&state.output)
            .font(Font::MONOSPACE)
            .wrapping(Wrapping::WordOrGlyph)
            .highlight("rust", Theme::InspiredGitHub)
            .on_action(Message::Editor)
            .height(Fill),
        row![
            button("Sync").on_press(Message::Show),
            button("Async").on_press(Message::Spawn),
        ]
        .spacing(8),
    ]
    .spacing(8)
    .into()
}

fn show_dialog(settings: &FileSettings) -> Task<Message> {
    build_file_dialog(settings)
        .map(|builder| builder.open_single_file())
        .map(|dialog| dialog.show().unwrap())
        .map(Message::Update)
}

fn spawn_dialog(settings: &FileSettings) -> Task<Message> {
    build_file_dialog(settings)
        .map(|builder| builder.open_single_file())
        .then(|dialog| Task::future(dialog.spawn()))
        .map(Result::unwrap)
        .map(Message::Update)
}
