use objc2_core_foundation::kCFUserNotificationDefaultResponse;

use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::ffi::mac::Alert;
use crate::Result;

impl MessageAlert {
    fn create(&self) -> Alert {
        Alert::new(&self.title, &self.text, self.level, false)
    }
}

impl DialogImpl for MessageAlert {
    fn show(self) -> Result<Self::Output> {
        self.create().display();
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        self.show()
    }
}

impl MessageConfirm {
    fn create(&self) -> Alert {
        Alert::new(&self.title, &self.text, self.level, true)
    }
}

impl DialogImpl for MessageConfirm {
    fn show(self) -> Result<Self::Output> {
        let response = self.create().display();
        Ok(response == kCFUserNotificationDefaultResponse)
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        self.show()
    }
}
