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
    use activation_context::ActivationContext;

    match ActivationContext::get() {
        Some(ctx) => ctx.with(f),
        None => f(),
    }
}

#[cfg(feature = "windows_visual_styles")]
mod activation_context {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::sync::LazyLock;

    use winapi::shared::minwindef::{DWORD, ULONG};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winbase::{ActivateActCtx, CreateActCtxW, DeactivateActCtx, ACTCTXW};
    use winapi::um::winnt::HANDLE;
    use winapi::um::winuser::MAKEINTRESOURCEW;

    const ACTCTX_FLAG_RESOURCE_NAME_VALID: DWORD = 0x008;
    const ACTCTX_FLAG_HMODULE_VALID: DWORD = 0x080;

    pub struct ActivationContext {
        handle: HANDLE,
    }

    unsafe impl Send for ActivationContext {}
    unsafe impl Sync for ActivationContext {}

    impl ActivationContext {
        pub fn with<T>(&self, f: impl Fn() -> T) -> T {
            let mut cookie = 0;
            unsafe { ActivateActCtx(self.handle, &mut cookie) };
            let result = f();
            unsafe { DeactivateActCtx(0, cookie) };

            result
        }

        pub fn get() -> Option<Self> {
            static INSTANCE: LazyLock<ActivationContext> = LazyLock::new(ActivationContext::new);

            let handle = INSTANCE.handle;
            if std::ptr::eq(handle, INVALID_HANDLE_VALUE) {
                return None;
            }

            Some(Self { handle })
        }

        fn new() -> ActivationContext {
            let module_name: Vec<u16> = OsStr::new("shell32.dll")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let context = ACTCTXW {
                cbSize: std::mem::size_of::<ACTCTXW>() as ULONG,
                hModule: unsafe { GetModuleHandleW(module_name.as_ptr()) },
                lpResourceName: MAKEINTRESOURCEW(124),
                dwFlags: ACTCTX_FLAG_HMODULE_VALID | ACTCTX_FLAG_RESOURCE_NAME_VALID,
                ..unsafe { std::mem::zeroed() }
            };

            ActivationContext {
                handle: unsafe { CreateActCtxW(&context) },
            }
        }
    }
}
