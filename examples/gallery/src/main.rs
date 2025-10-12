use iced::widget::{column, container, row};
use iced::Length::{Fill, FillPortion};
use iced::{Element, Task};

mod msg_alert;
mod msg_confirm;
mod open_dir;
mod open_multi;
mod open_single;
mod save_single;
mod settings;
mod utils;

use utils::cell;

#[derive(Debug, Default)]
struct State {
    settings: settings::State,
    open_single: open_single::State,
    open_multi: open_multi::State,
    open_dir: open_dir::State,
    save_single: save_single::State,
    msg_alert: msg_alert::State,
    msg_confirm: msg_confirm::State,
}

#[derive(Debug, Clone)]
enum Message {
    Settings(settings::Message),
    OpenSingle(open_single::Message),
    OpenMulti(open_multi::Message),
    OpenDir(open_dir::Message),
    SaveSingle(save_single::Message),
    MsgAlert(msg_alert::Message),
    MsgConfirm(msg_confirm::Message),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Settings(message) => {
            settings::update(&mut state.settings, message).map(Message::Settings)
        }
        Message::OpenSingle(message) => {
            open_single::update(&mut state.open_single, &state.settings.file, message)
                .map(Message::OpenSingle)
        }
        Message::OpenMulti(message) => {
            open_multi::update(&mut state.open_multi, &state.settings.file, message)
                .map(Message::OpenMulti)
        }
        Message::OpenDir(message) => {
            open_dir::update(&mut state.open_dir, &state.settings.file, message)
                .map(Message::OpenDir)
        }
        Message::SaveSingle(message) => {
            save_single::update(&mut state.save_single, &state.settings.file, message)
                .map(Message::SaveSingle)
        }
        Message::MsgAlert(message) => {
            msg_alert::update(&mut state.msg_alert, &state.settings.msg, message)
                .map(Message::MsgAlert)
        }
        Message::MsgConfirm(message) => {
            msg_confirm::update(&mut state.msg_confirm, &state.settings.msg, message)
                .map(Message::MsgConfirm)
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    row![
        container(settings::view(&state.settings).map(Message::Settings)).width(FillPortion(1)),
        column![
            row![
                cell(open_single::view(&state.open_single).map(Message::OpenSingle)),
                cell(open_multi::view(&state.open_multi).map(Message::OpenMulti)),
            ]
            .spacing(12)
            .width(Fill)
            .height(Fill),
            row![
                cell(open_dir::view(&state.open_dir).map(Message::OpenDir)),
                cell(save_single::view(&state.save_single).map(Message::SaveSingle)),
            ]
            .spacing(12)
            .width(Fill)
            .height(Fill),
            row![
                cell(msg_alert::view(&state.msg_alert).map(Message::MsgAlert)),
                cell(msg_confirm::view(&state.msg_confirm).map(Message::MsgConfirm)),
            ]
            .spacing(12)
            .width(Fill)
            .height(Fill),
        ]
        .spacing(12)
        .width(FillPortion(2))
        .height(Fill)
    ]
    .spacing(12)
    .padding(12)
    .into()
}

pub fn main() -> iced::Result {
    iced::run("Dialogs Gallery", update, view)
}
