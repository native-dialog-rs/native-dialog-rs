use std::cell::RefCell;

use crate::{ProgressDialog, ProgressHandle};

use super::Dialog;

impl Dialog for ProgressDialog<'_> {
    type Output = Box<RefCell<dyn ProgressHandle>>;
}
