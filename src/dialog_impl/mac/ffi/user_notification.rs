use std::ptr::null;
use core_foundation::{date::CFTimeInterval, base::{CFOptionFlags, SInt32, TCFType}, string::{CFString, CFStringRef, __CFString}, url::CFURLRef};

pub struct CFUserNotification<'a> {
    pub header: &'a str,
    pub message: &'a str,
    pub icon: usize,
    pub default_button_title: Option<&'a str>,
    pub alternate_button_title: Option<&'a str>,
}

impl<'a> CFUserNotification<'a> {
    pub fn display_alert(&self) -> i32 {
        unsafe {
            let mut response = 0;
            let mut dbt: CFStringRef = null();
            if let Some(s) = self.default_button_title {
                dbt = CFString::new(s).as_CFTypeRef() as *const __CFString;
            }
            let mut abt: CFStringRef = null();
            if let Some(s) = self.alternate_button_title {
                abt = CFString::new(s).as_CFTypeRef() as *const __CFString;
            }
            CFUserNotificationDisplayAlert(
                0 as f64,
                self.icon,
                null(),
                null(),
                null(),
                CFString::new(self.header).as_CFTypeRef() as *const __CFString,
                CFString::new(self.message).as_CFTypeRef() as *const __CFString,
                dbt,
                abt,
                null(),
                &mut response
            );
            response
        }
    }
}

extern {
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
        responseFlags: *mut SInt32) -> SInt32;
}
