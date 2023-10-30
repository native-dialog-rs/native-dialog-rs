use core_foundation::base::{CFOptionFlags, SInt32, TCFType};
use core_foundation::date::CFTimeInterval;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::CFURLRef;
use std::ptr::null;

pub struct UserNotificationAlert<'a> {
    pub header: &'a str,
    pub message: &'a str,
    pub icon: usize,
    pub confirm: bool,
}

impl<'a> UserNotificationAlert<'a> {
    pub fn display(&self) -> i32 {
        let default = CFString::from_static_string("Yes");
        let alternate = CFString::from_static_string("No");
        let header = CFString::new(self.header);
        let message = CFString::new(self.message);

        let mut response = 0;
        unsafe {
            CFUserNotificationDisplayAlert(
                0f64,
                self.icon,
                null(),
                null(),
                null(),
                header.as_CFTypeRef() as _,
                message.as_CFTypeRef() as _,
                match self.confirm {
                    true => default.as_CFTypeRef() as _,
                    false => null(),
                },
                match self.confirm {
                    true => alternate.as_CFTypeRef() as _,
                    false => null(),
                },
                null(),
                &mut response,
            );
        }

        response
    }
}

extern "C" {
    fn CFUserNotificationDisplayAlert(
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        iconURL: CFURLRef,
        soundURL: CFURLRef,
        localizationURL: CFURLRef,
        alertHeader: CFStringRef,
        alertMessage: CFStringRef,
        defaultButtonTitle: CFStringRef,
        alternateButtonTitle: CFStringRef,
        otherButtonTitle: CFStringRef,
        responseFlags: *mut SInt32,
    ) -> SInt32;
}
