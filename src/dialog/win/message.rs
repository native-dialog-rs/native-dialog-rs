use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm, UnsafeWindowHandle};
use crate::{MessageLevel, Result};

impl MessageAlert {
    fn create(&self) -> MessageBoxParams {
        MessageBoxParams {
            title: &self.title,
            text: &self.text,
            level: self.level,
            owner: self.owner,
            ask: false,
        }
    }
}

impl DialogImpl for MessageAlert {
    fn show(self) -> Result<Self::Output> {
        super::process_init();
        message_box(self.create())?;
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        self.show()
    }
}

impl MessageConfirm {
    fn create(&self) -> MessageBoxParams {
        MessageBoxParams {
            title: &self.title,
            text: &self.text,
            level: self.level,
            owner: self.owner,
            ask: true,
        }
    }
}

impl DialogImpl for MessageConfirm {
    fn show(self) -> Result<Self::Output> {
        super::process_init();
        message_box(self.create())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        self.show()
    }
}

pub struct MessageBoxParams<'a> {
    title: &'a str,
    text: &'a str,
    level: MessageLevel,
    owner: Option<UnsafeWindowHandle>,
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

    let owner = params
        .owner
        .and_then(|owner| unsafe { owner.as_win32() })
        .map(|handle| handle.hwnd.get() as HWND)
        .unwrap_or(null_mut());

    let text: Vec<u16> = OsStr::new(params.text)
        .encode_wide()
        .chain(once(0))
        .collect();

    let caption: Vec<u16> = OsStr::new(params.title)
        .encode_wide()
        .chain(once(0))
        .collect();

    let flags_type = if params.ask { MB_YESNO } else { MB_OK };
    let flags_icon = match params.level {
        MessageLevel::Info => MB_ICONINFORMATION,
        MessageLevel::Warning => MB_ICONWARNING,
        MessageLevel::Error => MB_ICONERROR,
    };

    let ret = super::with_visual_styles(|| unsafe {
        MessageBoxW(
            owner,
            text.as_ptr(),
            caption.as_ptr(),
            flags_type | flags_icon,
        )
    });

    match ret {
        0 => Err(std::io::Error::last_os_error().into()),
        x => Ok(x == IDYES),
    }
}
