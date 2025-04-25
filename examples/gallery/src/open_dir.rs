use std::path::PathBuf;

use iced::highlighter::Theme;
use iced::widget::text::Wrapping;
use iced::widget::text_editor::{Action, Content};
use iced::widget::{button, column, row, text_editor};
use iced::window::{get_oldest, run_with_handle};
use iced::Length::Fill;
use iced::{Element, Font, Task};
use native_dialog::{DialogBuilder, OpenSingleDir};

#[derive(Debug, Default)]
pub struct State {
    content: Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    Show,
    Spawn,
    Update(Option<PathBuf>),
    Editor(Action),
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Show => show_dialog(),
        Message::Spawn => spawn_dialog(),
        Message::Update(output) => {
            state.content = Content::with_text(&format!("{:?}", output));
            Task::none()
        }
        Message::Editor(action) => {
            if !action.is_edit() {
                state.content.perform(action);
            }
            Task::none()
        }
    }
}

pub fn view(state: &State) -> Element<Message> {
    column![
        "Open Single Dir",
        text_editor(&state.content)
            .font(Font::MONOSPACE)
            .wrapping(Wrapping::WordOrGlyph)
            .highlight("rs", Theme::InspiredGitHub)
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

fn create_dialog() -> Task<OpenSingleDir> {
    get_oldest().and_then(|id| {
        run_with_handle(id, |handle| {
            DialogBuilder::file().set_owner(&handle).open_single_dir()
        })
    })
}

fn show_dialog() -> Task<Message> {
    create_dialog()
        .map(|dialog| dialog.show().unwrap())
        .map(Message::Update)
}

fn spawn_dialog() -> Task<Message> {
    create_dialog()
        .then(|dialog| Task::future(dialog.spawn()))
        .map(Result::unwrap)
        .map(Message::Update)
}
