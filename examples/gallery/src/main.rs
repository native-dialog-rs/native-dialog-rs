use iced::widget::{column, container, row, Container};
use iced::Length::Fill;
use iced::{color, Border, Element, Task};

mod msg_alert;
mod msg_confirm;
mod open_dir;
mod open_multi;
mod open_single;
mod save_single;

#[derive(Debug, Default)]
struct State {
    open_single: open_single::State,
    open_multi: open_multi::State,
    open_dir: open_dir::State,
    save_single: save_single::State,
    msg_alert: msg_alert::State,
    msg_confirm: msg_confirm::State,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
enum Message {
    OpenSingle(open_single::Message),
    OpenMulti(open_multi::Message),
    OpenDir(open_dir::Message),
    SaveSingle(save_single::Message),
    MsgAlert(msg_alert::Message),
    MsgConfirm(msg_confirm::Message),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::OpenSingle(message) => {
            open_single::update(&mut state.open_single, message).map(Message::OpenSingle)
        }
        Message::OpenMulti(message) => {
            open_multi::update(&mut state.open_multi, message).map(Message::OpenMulti)
        }
        Message::OpenDir(message) => {
            open_dir::update(&mut state.open_dir, message).map(Message::OpenDir)
        }
        Message::SaveSingle(message) => {
            save_single::update(&mut state.save_single, message).map(Message::SaveSingle)
        }
        Message::MsgAlert(message) => {
            msg_alert::update(&mut state.msg_alert, message).map(Message::MsgAlert)
        }
        Message::MsgConfirm(message) => {
            msg_confirm::update(&mut state.msg_confirm, message).map(Message::MsgConfirm)
        }
    }
}

fn view(state: &State) -> Element<Message> {
    container(
        column![
            row![
                cell(open_single::view(&state.open_single).map(Message::OpenSingle)),
                cell(open_multi::view(&state.open_multi).map(Message::OpenMulti)),
                cell(open_dir::view(&state.open_dir).map(Message::OpenDir)),
            ]
            .spacing(12)
            .width(Fill)
            .height(Fill),
            row![
                cell(save_single::view(&state.save_single).map(Message::SaveSingle)),
                cell(msg_alert::view(&state.msg_alert).map(Message::MsgAlert)),
                cell(msg_confirm::view(&state.msg_confirm).map(Message::MsgConfirm)),
            ]
            .spacing(12)
            .width(Fill)
            .height(Fill),
        ]
        .spacing(12)
        .width(Fill)
        .height(Fill),
    )
    .padding(12)
    .width(Fill)
    .height(Fill)
    .into()
}

fn cell<T>(element: Element<T>) -> Container<T> {
    container(element)
        .width(Fill)
        .height(Fill)
        .padding(12)
        .style(|_| {
            container::Style::default()
                .background(color!(0xf5f5f5))
                .border(
                    Border::default()
                        .color(color!(0xd8d8d8))
                        .width(1)
                        .rounded(2),
                )
        })
}

pub fn main() -> iced::Result {
    iced::run("Dialogs Gallery", update, view)
}
