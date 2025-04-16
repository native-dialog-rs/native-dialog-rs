use raw_window_handle::RawWindowHandle;

use crate::dialog::{MessageAlert, MessageConfirm};
use crate::dialog_impl::DialogImpl;
use crate::{MessageType, Result};

impl<'a> DialogImpl for MessageAlert<'a> {
    type Impl = MessageBoxParams<'a>;

    fn create(&self) -> Self::Impl {
        MessageBoxParams {
            title: self.title,
            text: self.text,
            typ: self.typ,
            owner: self.owner,
            ask: false,
        }
    }

    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();
        message_box(self.create())?;
        Ok(())
    }

    async fn spawn(&mut self) -> Result<Self::Output> {
        self.show()
    }
}

impl<'a> DialogImpl for MessageConfirm<'a> {
    type Impl = MessageBoxParams<'a>;

    fn create(&self) -> Self::Impl {
        MessageBoxParams {
            title: self.title,
            text: self.text,
            typ: self.typ,
            owner: self.owner,
            ask: true,
        }
    }

    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();
        message_box(self.create())
    }

    async fn spawn(&mut self) -> Result<Self::Output> {
        self.show()
    }
}

struct MessageBoxParams<'a> {
    title: &'a str,
    text: &'a str,
    typ: MessageType,
    owner: Option<RawWindowHandle>,
    ask: bool,
}

fn message_box(params: MessageBoxParams) -> Result<bool> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::shared::windef::HWND;
    use winapi::um::winuser::{
        MessageBoxW, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MB_YESNO,
    };

    let owner = match params.owner {
        Some(RawWindowHandle::Win32(handle)) => handle.hwnd as HWND,
        _ => null_mut(),
    };

    let text: Vec<u16> = OsStr::new(params.text)
        .encode_wide()
        .chain(once(0))
        .collect();

    let caption: Vec<u16> = OsStr::new(params.title)
        .encode_wide()
        .chain(once(0))
        .collect();

    let u_type = match params.typ {
        MessageType::Info => MB_ICONINFORMATION,
        MessageType::Warning => MB_ICONWARNING,
        MessageType::Error => MB_ICONERROR,
    } | if params.ask { MB_YESNO } else { MB_OK };

    let ret = super::with_visual_styles(|| unsafe {
        MessageBoxW(owner, text.as_ptr(), caption.as_ptr(), u_type)
    });

    match ret {
        0 => Err(std::io::Error::last_os_error().into()),
        x => Ok(x == IDYES),
    }
}
