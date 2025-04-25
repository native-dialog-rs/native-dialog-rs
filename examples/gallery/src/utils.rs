use iced::widget::{container, text, Container, Text};
use iced::window::raw_window_handle::WindowHandle;
use iced::window::{get_oldest, run_with_handle};
use iced::Length::Fill;
use iced::{color, Border, Element, Task};
use native_dialog::{
    DialogBuilder, FileDialogBuilder, FileFilter, MessageDialogBuilder, MessageLevel,
};
use saphyr::{LoadableYamlNode, Yaml};

use crate::settings::{FileSettings, MsgSettings};

pub fn zip<T, U>(left: Task<T>, right: Task<U>) -> Task<(T, U)>
where
    T: Send + 'static,
    U: Send + 'static,
{
    let (send_left, recv_left) = async_channel::bounded(1);
    let (send_right, recv_right) = async_channel::bounded(1);

    Task::chain(
        left.then(move |value| {
            let send = send_left.clone();
            Task::future(async move { send.try_send(value).ok() })
        }),
        right.then(move |value| {
            let send = send_right.clone();
            Task::future(async move { send.try_send(value).ok() })
        }),
    )
    .collect()
    .then(move |_| {
        let recv_left = recv_left.clone();
        let recv_right = recv_right.clone();

        Task::future(async move {
            let left = recv_left.recv().await.unwrap();
            let right = recv_right.recv().await.unwrap();
            (left, right)
        })
    })
}

pub fn with_main_window<T, U, F>(value: T, f: F) -> Task<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T, WindowHandle) -> U + Copy + Send + 'static,
{
    zip(get_oldest(), Task::done(value)).then(move |(id, value)| match id {
        None => Task::none(),
        Some(id) => run_with_handle(id, move |handle| f(value, handle)),
    })
}

pub fn parse_filters(text: &str) -> Option<Vec<FileFilter>> {
    let yaml = Yaml::load_from_str(text).ok()?;
    let dict = yaml.first()?.as_mapping()?;

    dict.into_iter()
        .map(|(key, value)| {
            let name = key.as_str()?;
            let extensions = value
                .as_sequence()?
                .iter()
                .map(|value| value.as_str())
                .collect::<Option<Vec<_>>>()?;

            FileFilter::new(name, &extensions)
        })
        .collect()
}

pub fn build_file_dialog(settings: &FileSettings) -> Task<FileDialogBuilder> {
    let Some(filters) = parse_filters(&settings.filters.text()) else {
        return with_main_window((), |_, window| {
            DialogBuilder::message()
                .set_level(MessageLevel::Error)
                .set_title("Invalid File Filters")
                .set_text("Please check the dialog settings on the left side.")
                .set_owner(&window)
                .alert()
                .show()
        })
        .discard();
    };

    let builder = DialogBuilder::file()
        .set_title(&settings.title)
        .set_location(&settings.location)
        .add_filters(filters);

    with_main_window(builder, |builder, window| builder.set_owner(&window))
}

pub fn build_msg_dialog(settings: &MsgSettings) -> Task<MessageDialogBuilder> {
    let builder = DialogBuilder::message()
        .set_level(settings.level)
        .set_title(&settings.title)
        .set_text(settings.text.text());

    with_main_window(builder, |builder, window| builder.set_owner(&window))
}

pub fn cell<T>(element: Element<T>) -> Container<T> {
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

pub fn label(content: &str) -> Text {
    text(content).size(14).color(color!(0x707070))
}
