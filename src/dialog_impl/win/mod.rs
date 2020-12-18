mod file;
mod message;

fn process_init() {
    use std::sync::Once;

    static INIT: Once = Once::new();

    #[allow(unused_unsafe)]
    INIT.call_once(|| unsafe {
        #[cfg(feature = "windows_dpi_awareness")]
        winapi::um::winuser::SetProcessDPIAware();
    });
}

#[cfg(not(feature = "windows_visual_styles"))]
#[inline(always)]
fn with_visual_styles<T>(f: impl Fn() -> T) -> T {
    f()
}

#[cfg(feature = "windows_visual_styles")]
#[inline(always)]
fn with_visual_styles<T>(f: impl Fn() -> T) -> T {
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::winbase::{ActivateActCtx, DeactivateActCtx};

    let handle = act_ctx::get_handle();

    if handle != INVALID_HANDLE_VALUE {
        let mut cookie = 0;
        unsafe { ActivateActCtx(handle, &mut cookie) };
        let result = f();
        unsafe { DeactivateActCtx(0, cookie) };
        result
    } else {
        f()
    }
}

#[cfg(feature = "windows_visual_styles")]
mod act_ctx {
    use winapi::um::winnt::HANDLE;

    struct ActCtxHandle {
        handle: HANDLE,
    }

    unsafe impl Send for ActCtxHandle {}
    unsafe impl Sync for ActCtxHandle {}

    fn init() -> ActCtxHandle {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use winapi::shared::minwindef::{DWORD, ULONG};
        use winapi::um::libloaderapi::GetModuleHandleW;
        use winapi::um::winbase::{CreateActCtxW, ACTCTXW};
        use winapi::um::winuser::MAKEINTRESOURCEW;

        const ACTCTX_FLAG_RESOURCE_NAME_VALID: DWORD = 0x008;
        const ACTCTX_FLAG_HMODULE_VALID: DWORD = 0x080;

        let mut module_name: Vec<u16> = OsStr::new("shell32.dll")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut context = ACTCTXW {
            cbSize: std::mem::size_of::<ACTCTXW> as ULONG,
            hModule: unsafe { GetModuleHandleW(module_name.as_mut_ptr()) },
            lpResourceName: MAKEINTRESOURCEW(124),
            dwFlags: ACTCTX_FLAG_HMODULE_VALID | ACTCTX_FLAG_RESOURCE_NAME_VALID,
            ..unsafe { std::mem::zeroed() }
        };

        ActCtxHandle {
            handle: unsafe { CreateActCtxW(&mut context) },
        }
    }

    pub fn get_handle() -> HANDLE {
        use once_cell::sync::OnceCell;

        static ACT_CTX_HANDLE: OnceCell<ActCtxHandle> = OnceCell::new();

        ACT_CTX_HANDLE.get_or_init(init).handle
    }
}
