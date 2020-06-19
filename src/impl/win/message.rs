use crate::{Dialog, MessageAlert, MessageConfirm, MessageType, Result};

impl Dialog for MessageAlert<'_> {
    type Output = ();

    fn show(self) -> Result<Self::Output> {
        super::process_init();

        message_box(MessageBoxParams {
            title: self.title,
            text: self.text,
            typ: self.typ,
            ask: false,
        })?;
        Ok(())
    }
}

impl Dialog for MessageConfirm<'_> {
    type Output = bool;

    fn show(self) -> Result<Self::Output> {
        super::process_init();

        message_box(MessageBoxParams {
            title: self.title,
            text: self.text,
            typ: self.typ,
            ask: true,
        })
    }
}

struct MessageBoxParams<'a> {
    title: &'a str,
    text: &'a str,
    typ: MessageType,
    ask: bool,
}

fn message_box(params: MessageBoxParams) -> Result<bool> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{
        MessageBoxW, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MB_YESNO,
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
        MessageBoxW(null_mut(), text.as_ptr(), caption.as_ptr(), u_type)
    });

    match ret {
        0 => Err(std::io::Error::last_os_error())?,
        x => Ok(x == IDYES),
    }
}
