use std::cell::RefCell;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::mpsc;

use raw_window_handle::RawWindowHandle;
use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::PostMessageW;

use crate::{Error, Result};
use crate::{ProgressDialog, ProgressHandle};
use crate::dialog::DialogImpl;

struct AtomicHandles {
    hwnd: AtomicPtr<c_void>,
    hinstance: AtomicPtr<c_void>,
}

struct HwndWrapper {
    hwnd: HWND,
}

// SAFETY: HWNDs are OK to send between threads.
// Be sure we don't use the pointer for anything else.
unsafe impl Send for HwndWrapper {}

struct Params {
    title: String,
    text: String,
    owner: Option<AtomicHandles>,
}

impl<'a> DialogImpl for ProgressDialog<'a> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let params = Params {
            title: self.title.into(),
            text: self.text.into(),
            owner: self.owner.and_then(|raw_handle| match raw_handle {
                RawWindowHandle::Windows(win) => Some(AtomicHandles {
                    hwnd: AtomicPtr::from(win.hwnd),
                    hinstance: AtomicPtr::from(win.hinstance),
                }),
                _ => None,
            }),
        };

        let (hwnd_tx, hwnd_rx) = mpsc::channel::<HwndWrapper>();
        let (res_sender, res_recv) = mpsc::channel::<bool>();
        std::thread::spawn(move || {
            let res = open_task_dialog(params, hwnd_tx);
            if let Ok(cancel) = res {
                res_sender.send(cancel).ok();
            }
        });

        let hwnd = hwnd_rx
            .recv()
            .map_err(|_e| {
                Error::ImplementationError("Window thread exited early (crashed?)".into())
            })?
            .hwnd;

        unsafe {
            use winapi::shared::minwindef::MAKELONG;
            use winapi::um::commctrl::TDM_SET_PROGRESS_BAR_RANGE;

            // 0-1000 resolution
            PostMessageW(
                hwnd,
                TDM_SET_PROGRESS_BAR_RANGE,
                0,
                MAKELONG(0, 1000) as isize,
            );
        };

        let handle = WindowsProgressHandle {
            hwnd,
            cancel_check: res_recv,
        };
        Ok(Box::new(RefCell::new(handle)))
    }
}

fn str_to_pointer(text: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(text).encode_wide().chain(once(0)).collect()
}

extern "system" fn task_cb(
    hwnd: HWND,
    msg: u32,
    wparam: usize,
    lparam: isize,
    userdata: isize,
) -> i32 {
    let cb_ref: &&dyn Fn(HWND, UINT, WPARAM, LPARAM) -> i32 = unsafe {
        let double_ref = userdata as *const c_void;
        &*(double_ref as *const _)
    };

    cb_ref(hwnd, msg, wparam, lparam)
}

fn open_task_dialog(settings: Params, handle: mpsc::Sender<HwndWrapper>) -> Result<bool> {
    use winapi::shared::minwindef::HINSTANCE;
    use winapi::shared::winerror::{E_FAIL, E_INVALIDARG, E_OUTOFMEMORY, S_FALSE, S_OK};
    use winapi::um::commctrl::{
        TaskDialogIndirect, TASKDIALOGCONFIG, TDCBF_CANCEL_BUTTON, TDF_SHOW_PROGRESS_BAR,
        TDN_BUTTON_CLICKED, TDN_CREATED,
    };
    use winapi::um::winuser::IDCANCEL;

    let cb = |hwnd: HWND, msg: UINT, wparam: WPARAM, _lparam: LPARAM| match msg {
        TDN_CREATED => {
            // No error handling here, we're inside a windows callback
            // If this errors, the other side has hung up and is probably crashing.
            // Windows might explode if we panic?
            handle.send(HwndWrapper { hwnd }).ok();
            S_OK
        }
        TDN_BUTTON_CLICKED => {
            if wparam == IDCANCEL as usize {
                S_OK
            } else {
                S_FALSE
            }
        }
        _ => S_OK,
    };

    let cb_trait: &dyn Fn(HWND, UINT, WPARAM, LPARAM) -> i32 = &cb;
    let cb_trait_ref = &cb_trait;
    let lparam = {
        let cb_ptr = cb_trait_ref as *const _ as *const c_void;
        cb_ptr as isize
    };

    let handle = settings.owner;

    let title = str_to_pointer(&settings.title);
    let content = str_to_pointer(&settings.text);
    let config = TASKDIALOGCONFIG {
        cbSize: std::mem::size_of::<TASKDIALOGCONFIG>() as u32,
        hwndParent: handle
            .as_ref()
            .map(|h| h.hwnd.load(Ordering::Relaxed))
            .unwrap_or(null_mut()) as HWND,
        hInstance: handle
            .as_ref()
            .map(|h| h.hinstance.load(Ordering::Relaxed))
            .unwrap_or(null_mut()) as HINSTANCE,
        dwFlags: TDF_SHOW_PROGRESS_BAR,
        dwCommonButtons: TDCBF_CANCEL_BUTTON,
        pszWindowTitle: title.as_ptr(),
        u1: Default::default(),
        pszMainInstruction: content.as_ptr(),
        pfCallback: Some(task_cb),
        lpCallbackData: lparam,
        ..Default::default()
    };

    let (retval, result) = super::with_visual_styles(|| unsafe {
        let mut result = 0;
        (
            TaskDialogIndirect(&config, &mut result, null_mut(), null_mut()),
            result,
        )
    });

    match retval {
        S_OK => Ok(result == IDCANCEL),
        E_OUTOFMEMORY => Err(Error::ImplementationError("Out of memory".into())),
        E_INVALIDARG => Err(Error::ImplementationError("Invalid argument".into())),
        E_FAIL => Err(Error::ImplementationError(
            "Generic failure opening task dialog".into(),
        )),
        _ => Err(Error::ImplementationError("Unknown error".into())),
    }
}

struct WindowsProgressHandle {
    hwnd: HWND,
    cancel_check: mpsc::Receiver<bool>,
}

impl ProgressHandle for WindowsProgressHandle {
    fn set_progress(&mut self, percent: f32) -> Result<()> {
        use winapi::um::commctrl::TDM_SET_PROGRESS_BAR_POS;

        if percent < 0.0 || percent > 100.0 {
            return Err(Error::InvalidPercentage(percent));
        }

        let pos = (percent * 10.0) as usize;
        unsafe { PostMessageW(self.hwnd, TDM_SET_PROGRESS_BAR_POS, pos, 0) };

        Ok(())
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        use winapi::um::commctrl::{TDE_MAIN_INSTRUCTION, TDM_UPDATE_ELEMENT_TEXT};

        let content = str_to_pointer(text);

        unsafe {
            PostMessageW(
                self.hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_MAIN_INSTRUCTION as usize,
                content.as_ptr() as isize,
            )
        };

        Ok(())
    }

    fn check_cancelled(&mut self) -> Result<bool> {
        use std::sync::mpsc::TryRecvError;

        match self.cancel_check.try_recv() {
            Ok(cancel) => Ok(cancel),
            Err(err) => match err {
                TryRecvError::Empty => Ok(false),
                TryRecvError::Disconnected => {
                    Err(Error::ImplementationError("Window disconnected".into()))
                }
            },
        }
    }

    fn close(&mut self) -> Result<()> {
        use winapi::um::commctrl::TDM_CLICK_BUTTON;
        use winapi::um::winuser::IDCANCEL;

        unsafe { PostMessageW(self.hwnd, TDM_CLICK_BUTTON, IDCANCEL as usize, 0) };
        Ok(())
    }
}
