mod file;
mod message;

fn process_init() {
    use std::sync::Once;

    static INIT: Once = Once::new();

    INIT.call_once(|| {
        #[cfg(feature = "windows_dpi_awareness")]
        enable_dpi_awareness();

        #[cfg(feature = "windows_visual_styles")]
        enable_visual_styles();
    });
}

#[cfg(feature = "windows_dpi_awareness")]
fn enable_dpi_awareness() {
    use winapi::um::winuser::SetProcessDPIAware;

    unsafe { SetProcessDPIAware() };
}

#[cfg(feature = "windows_visual_styles")]
fn enable_visual_styles() {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winapi::shared::minwindef::{DWORD, ULONG};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winbase::{ActivateActCtx, CreateActCtxW, ACTCTXW};
    use winapi::um::winuser::MAKEINTRESOURCEW;

    const ACTCTX_FLAG_RESOURCE_NAME_VALID: DWORD = 0x008;
    const ACTCTX_FLAG_HMODULE_VALID: DWORD = 0x080;
    const ACTCTX_FLAG_SET_PROCESS_DEFAULT: DWORD = 0x010;

    let mut module_name: Vec<u16> = OsStr::new("shell32.dll")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut context = ACTCTXW {
        cbSize: std::mem::size_of::<ACTCTXW> as ULONG,
        hModule: unsafe { GetModuleHandleW(module_name.as_mut_ptr()) },
        lpResourceName: MAKEINTRESOURCEW(124),
        dwFlags: ACTCTX_FLAG_HMODULE_VALID
            | ACTCTX_FLAG_RESOURCE_NAME_VALID
            | ACTCTX_FLAG_SET_PROCESS_DEFAULT,
        ..unsafe { std::mem::zeroed() }
    };

    let handle = unsafe { CreateActCtxW(&mut context) };

    if handle != INVALID_HANDLE_VALUE {
        let mut cookie = 0;
        unsafe { ActivateActCtx(handle, &mut cookie) };
    }
}
