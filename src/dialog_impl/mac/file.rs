use super::ffi::cocoa::{
    INSColor, INSOpenPanel, INSPopUpButton, INSSavePanel, INSStackView, INSTextField, INSUrl,
    INSWindow, NSColor, NSEdgeInsets, NSOpenPanel, NSPopUpButton, NSSavePanel, NSStackView,
    NSStackViewGravity, NSTextField, NSUserInterfaceLayoutOrientation,
};
use super::ffi::{DropdownAction, IDropdownAction};
use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::{Filter, Result};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use objc_foundation::{INSArray, INSMutableArray, INSObject, INSString, NSMutableArray, NSString};
use objc_id::Id;

impl DialogImpl for OpenSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(false);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_file_types(get_all_allowed_types(&self.filters));

        let owner = self.owner.and_then(INSWindow::from_raw_handle);
        match panel.run_modal(owner) {
            Ok(urls) => {
                let url = urls.first_object().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            Err(_) => Ok(None),
        }
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(true);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_file_types(get_all_allowed_types(&self.filters));

        let owner = self.owner.and_then(INSWindow::from_raw_handle);
        match panel.run_modal(owner) {
            Ok(urls) => Ok(urls.to_vec().into_iter().map(INSUrl::to_path_buf).collect()),
            Err(_) => Ok(vec![]),
        }
    }
}

impl DialogImpl for OpenSingleDir<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_can_choose_files(false);
        panel.set_can_choose_directories(true);
        panel.set_allows_multiple_selection(false);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        let owner = self.owner.and_then(INSWindow::from_raw_handle);
        match panel.run_modal(owner) {
            Ok(urls) => {
                let url = urls.first_object().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            Err(_) => Ok(None),
        }
    }
}

impl DialogImpl for SaveSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSSavePanel::save_panel().share();

        panel.set_can_create_directories(false);
        panel.set_extension_hidden(false);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        // If there are filters specified, show a dropdown on the panel
        let res = if let Some(first_filter) = self.filters.first() {
            let action_target = DropdownAction::new().share();
            action_target.set_save_panel(panel.clone());

            unsafe { action_target.set_filters(&self.filters) };

            let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(480.0, 0.0));
            let dropdown = NSPopUpButton::new_with_frame(frame, false);
            dropdown.add_items_with_titles(get_file_type_dropdown_items(&self.filters));
            dropdown.select_item_at(0);
            dropdown.set_action(sel!(onItemSelected:));
            dropdown.set_target(action_target.clone());

            let label = NSTextField::label_with_string("File Type: ");
            label.set_text_color(NSColor::secondary_label_color());

            let stack = NSStackView::new();
            // Edge insets in specific axis are only enforced when hugging priority >= 500
            // See https://stackoverflow.com/questions/54533509/nsstackview-edgeinsets-gets-ignored
            stack.set_hugging_priority(500.0, NSUserInterfaceLayoutOrientation::Vertical);
            stack.set_hugging_priority(500.0, NSUserInterfaceLayoutOrientation::Horizontal);
            stack.set_edge_insets(NSEdgeInsets::new(16.0, 20.0, 16.0, 20.0));
            stack.add_view_in_gravity(label, NSStackViewGravity::Center);
            stack.add_view_in_gravity(dropdown, NSStackViewGravity::Center);

            panel.set_allowed_file_types(first_filter.extensions);
            panel.set_accessory_view(stack);

            let owner = self.owner.and_then(INSWindow::from_raw_handle);
            let res = panel.run_modal(owner);

            unsafe { action_target.set_filters(std::ptr::null()) };

            res
        } else {
            let owner = self.owner.and_then(INSWindow::from_raw_handle);
            panel.run_modal(owner)
        };

        match res {
            Ok(url) => Ok(Some(url.to_path_buf())),
            Err(_) => Ok(None),
        }
    }
}

fn get_all_allowed_types(filters: &[Filter<'_>]) -> Id<NSMutableArray<NSString>> {
    let mut extensions = NSMutableArray::new();
    for filter in filters {
        for ext in filter.extensions {
            let s = NSString::from_str(ext);
            extensions.add_object(s);
        }
    }
    extensions
}

fn get_file_type_dropdown_items(filters: &[Filter<'_>]) -> Id<NSMutableArray<NSString>> {
    let mut titles = NSMutableArray::new();
    for filter in filters {
        let extensions: Vec<String> = filter
            .extensions
            .iter()
            .map(|s| format!("*.{}", s))
            .collect();
        let title = format!("{} ({})", filter.description, extensions.join(" "));
        titles.add_object(NSString::from_str(&title));
    }
    titles
}
