use sdl2::messagebox::{ButtonData, MessageBoxButtonFlag, MessageBoxColorScheme, MessageBoxFlag};

use crate::{
    dialog::{DialogImpl, FallbackMessageAlert, FallbackMessageConfirm},
    MessageType,
};

const SDL_DIALOG_SCHEME: MessageBoxColorScheme = MessageBoxColorScheme {
    background: (255, 255, 255),
    text: (0, 0, 0),
    button_border: (1, 1, 1),
    button_background: (200, 200, 200),
    button_selected: (34, 111, 163),
};

impl DialogImpl for FallbackMessageConfirm<'_> {
    fn show(&mut self) -> crate::Result<Self::Output> {
        let window = create_sdl_window()?;

        let flags = to_sdl_dialog_type(&self.typ);
        let button1 = ButtonData {
            flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
            button_id: 1,
            text: "Yes",
        };
        let button2 = ButtonData {
            flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
            button_id: 2,
            text: "No",
        };
        let buttons = [button1, button2];

        match sdl2::messagebox::show_message_box(
            flags,
            &buttons,
            self.title,
            self.text,
            &window,
            SDL_DIALOG_SCHEME,
        ) {
            Ok(value) => match value {
                sdl2::messagebox::ClickedButton::CloseButton => Ok(false),
                sdl2::messagebox::ClickedButton::CustomButton(btn) => match btn.button_id == 1 {
                    true => Ok(true),
                    false => Ok(false),
                },
            },
            Err(e) => Err(crate::Error::ImplementationError(e.to_string())),
        }
    }
}

impl DialogImpl for FallbackMessageAlert<'_> {
    fn show(&mut self) -> crate::Result<Self::Output> {
        let window = create_sdl_window()?;

        let flags = to_sdl_dialog_type(&self.typ);
        let button1 = ButtonData {
            flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
            button_id: 1,
            text: "OK",
        };
        let buttons = [button1];

        match sdl2::messagebox::show_message_box(
            flags,
            &buttons,
            self.title,
            self.text,
            &window,
            SDL_DIALOG_SCHEME,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::Error::ImplementationError(e.to_string())),
        }
    }
}

fn create_sdl_window() -> crate::Result<sdl2::video::Window> {
    let sdl_context = sdl2::init().map_err(crate::Error::ImplementationError)?;
    let video_subsystem = sdl_context
        .video()
        .map_err(crate::Error::ImplementationError)?;

    // Hiding this window also means the dialog window is hidden and not visible in taskbars.
    // Making the size of the window none is an okay solution, but it still has a main window.
    match video_subsystem.window("", 0, 0).position_centered().build() {
        Ok(window) => Ok(window),
        Err(err) => Err(crate::Error::ImplementationError(err.to_string())),
    }
}

fn to_sdl_dialog_type(typ: &MessageType) -> MessageBoxFlag {
    match typ {
        MessageType::Info => MessageBoxFlag::INFORMATION,
        MessageType::Warning => MessageBoxFlag::WARNING,
        MessageType::Error => MessageBoxFlag::ERROR,
    }
}
