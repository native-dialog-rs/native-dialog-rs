use std::path::PathBuf;

use kas::config::Config;
use kas::dir::Directions;
use kas::prelude::*;
use kas::text::fonts::FontSelector;
use kas::theme::{FlatTheme, MarginStyle, TextClass};
use kas::widgets::{column, Adapt, Button, EditBox};
use native_dialog::DialogBuilder;

#[derive(Debug, Clone, Default)]
struct State {
    id: Id,
    path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct Configure(Id);

#[derive(Debug, Clone)]
struct Update(Option<PathBuf>);

#[derive(Debug, Clone)]
struct Pick;

fn view() -> impl Widget<Data = ()> {
    let tree = column![
        EditBox::string(|state: &State| format!("{:?}", state.path))
            .with_multi_line(true)
            .with_lines(16, 16)
            .with_width_em(36.0, 36.0),
        Button::label_msg("Pick a File", Pick).map_any(),
    ]
    .margins(Directions::all(), MarginStyle::Em(1.0))
    .on_configure(|ctx, widget| {
        ctx.send(widget.id(), Configure(widget.id()));
    });

    Adapt::new(tree, State::default())
        .on_message(|_, state, Configure(id)| state.id = id)
        .on_message(|ctx, state, Update(path)| {
            state.path = path.clone();

            let dialog = DialogBuilder::message()
                .set_owner(ctx.winit_window().unwrap())
                .set_title("Update")
                .set_text(format!("{:?}", path))
                .alert();

            ctx.push_async(state.id.clone(), async {
                dialog.spawn().await.unwrap();
            })
        })
        .on_message(|ctx, state, Pick| {
            let dialog = DialogBuilder::file()
                .set_owner(ctx.winit_window().unwrap())
                .add_filter("PNG", ["png"])
                .add_filter("JPEG", ["jpg", "jpeg"])
                .save_single_file();

            ctx.push_async(state.id.clone(), async {
                Update(dialog.spawn().await.unwrap())
            });
        })
}

fn config() -> Config {
    let mut selector = FontSelector::new();
    selector.set_families(vec!["monospace".into()]);

    let mut config = Config::default();
    config.font.size = 24.0;
    config.font.fonts.insert(TextClass::Edit(true), selector);

    config
}

fn main() -> kas::runner::Result<()> {
    let app = kas::runner::Default::with_theme(FlatTheme::default())
        .with_config(config())
        .build(())?;

    let window = Window::new(view(), "A fantastic window!");
    app.with(window).run()
}
