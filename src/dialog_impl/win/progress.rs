use std::cell::RefCell;
use std::ptr::null_mut;

use raw_window_handle::RawWindowHandle;
use winapi::shared::windef::HWND;
use winapi::um::winuser::SendMessageW;

use crate::{Error, Result};
use crate::{ProgressDialog, ProgressHandle};
use crate::dialog::DialogImpl;

impl<'a> DialogImpl for ProgressDialog<'a> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        use std::ffi::OsStr;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;
        use std::ptr::null_mut;
        use winapi::um::commctrl::{PBM_SETRANGE, PBM_SETSTEP, PROGRESS_CLASS};
        use winapi::um::winuser::{CreateWindowExW, WS_BORDER, WS_POPUP, WS_VISIBLE};

        let class: Vec<u16> = OsStr::new(PROGRESS_CLASS)
            .encode_wide()
            .chain(once(0))
            .collect();

        let caption: Vec<u16> = OsStr::new(self.title)
            .encode_wide()
            .chain(once(0))
            .collect();

        let handle = self.owner.map(|hndl| (hndlr as RawWindowHandle::Windows));

        let hwnd = super::with_visual_styles(|| unsafe {
            CreateWindowExW(
                0,
                class.as_ptr(),
                caption.as_ptr(),
                WS_BORDER | WS_POPUP | WS_VISIBLE,
                0,
                0,
                300,
                150,
                handle.map(|h| h.hwnd).unwrap_or(null_mut()),
                null_mut(),
                handle.map(|h| h.instance).unwrap_or(null_mut()),
                null_mut(),
            )
        });

        unsafe {
            // This is all integers so set upper limit to 1000 for smoothness
            SendMessageW(hwnd, PBM_SETRANGE, 0, 1000);
            SendMessageW(hwnd, PBM_SETSTEP, 1, 0);
        };

        let handle = WindowsProgressHandle { hwnd };
        Ok(Box::new(RefCell::new(handle)))
    }
}

struct WindowsProgressHandle {
    hwnd: HWND,
}

impl ProgressHandle for WindowsProgressHandle {
    fn set_progress(&mut self, percent: f32) -> Result<()> {
        use winapi::um::commctrl::PBM_SETPOS;

        if percent < 0.0 || percent > 100.0 {
            return Err(Error::InvalidPercentage(percent));
        }

        let pos = (percent * 10.0) as usize;
        let ret = unsafe { SendMessageW(self.hwnd, PBM_SETPOS, pos, 0) };

        match ret {
            0 => Err(std::io::Error::last_os_error().into()),
            _ => Ok(()),
        }
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        // Currently a noop because the progress window doesn't show text :(
        // Maybe put it in the title bar?
        Ok(())
    }

    fn check_cancelled(&mut self) -> Result<bool> {
        use winapi::um::winuser::{PeekMessageW, LPMSG, MSG, PM_REMOVE, WM_CLOSE};

        let msg: LPMSG = null_mut();
        let ret = unsafe { PeekMessageW(msg, self.hwnd, WM_CLOSE, WM_CLOSE, PM_REMOVE) };

        let res = match ret {
            0 => false,
            _ => {
                self.close(); // clean up!!
                true
            }
        };

        Ok(res)
    }

    fn close(&mut self) -> Result<()> {
        use winapi::um::winuser::DestroyWindow;

        let ret = unsafe { DestroyWindow(self.hwnd) };
        match ret {
            0 => Err(std::io::Error::last_os_error().into()),
            _ => Ok(()),
        }
    }
}
