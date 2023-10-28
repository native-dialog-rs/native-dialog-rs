use objc_foundation::{INSObject, INSString, NSString};
use objc_id::Id;

pub trait IUTType: INSObject {
    fn from_extension(s: &str) -> Id<Self> {
        unsafe {
            let s = NSString::from_str(s);
            let ptr = msg_send![class!(UTType), typeWithFilenameExtension: s];
            Id::from_ptr(ptr)
        }
    }
}

object_struct!(UTType);

impl IUTType for UTType {}
