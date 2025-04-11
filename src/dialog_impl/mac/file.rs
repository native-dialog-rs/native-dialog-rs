use super::ffi::cocoa::{
    INSColor, INSOpenPanel, INSPopUpButton, INSSavePanel, INSStackView, INSTextField, INSWindow,
    INSURL,
};
use super::ffi::DropdownAction;
use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::{Filter, Result};
use objc2::rc::Id;
use objc2::sel;
use objc2_app_kit::{
    NSColor, NSLayoutConstraintOrientation, NSOpenPanel, NSPopUpButton, NSSavePanel, NSStackView,
    NSStackViewGravity, NSTextField,
};
use objc2_foundation::{NSEdgeInsets, NSMutableArray, NSPoint, NSRect, NSSize, NSString};

impl DialogImpl for OpenSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_title(self.title);
        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(false);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_extensions(&all_extensions(&self.filters));

        let owner = self.owner.and_then(INSWindow::from_raw_handle);
        match panel.run_modal(owner) {
            Ok(urls) => {
                let url = urls.first().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            Err(_) => Ok(None),
        }
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_title(self.title);
        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(true);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_extensions(&all_extensions(&self.filters));

        let owner = self.owner.and_then(INSWindow::from_raw_handle);
        match panel.run_modal(owner) {
            Ok(urls) => Ok(urls.to_vec().into_iter().map(INSURL::to_path_buf).collect()),
            Err(_) => Ok(vec![]),
        }
    }
}

impl DialogImpl for OpenSingleDir<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_title(self.title);
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
                let url = urls.first().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            Err(_) => Ok(None),
        }
    }
}

impl DialogImpl for SaveSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSSavePanel::save_panel();

        panel.set_title(self.title);
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
            let action_target = DropdownAction::new_with_save_panel(&panel);

            unsafe { action_target.set_filters(&self.filters) };

            let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(480.0, 0.0));
            let dropdown = NSPopUpButton::new_with_frame(frame, false);
            dropdown.add_items_with_titles(&file_type_dropdown_items(&self.filters));
            dropdown.select_item_at(0);
            dropdown.set_action(sel!(onItemSelected:));
            dropdown.set_target(&action_target);

            let label = NSTextField::label_with_string("File Type: ");
            label.set_text_color(&NSColor::secondary_label_color());

            let stack = NSStackView::new_empty();
            // Edge insets in specific axis are only enforced when hugging priority >= 500
            // See https://stackoverflow.com/questions/54533509/nsstackview-edgeinsets-gets-ignored
            stack.set_hugging_priority(500.0, NSLayoutConstraintOrientation::Vertical);
            stack.set_hugging_priority(500.0, NSLayoutConstraintOrientation::Horizontal);
            stack.set_edge_insets(NSEdgeInsets {
                top: 16.0,
                left: 20.0,
                bottom: 16.0,
                right: 20.0,
            });
            stack.add_view_in_gravity(&label, NSStackViewGravity::Center);
            stack.add_view_in_gravity(&dropdown, NSStackViewGravity::Center);

            panel.set_allowed_extensions(first_filter.extensions);
            panel.set_accessory_view(&stack);

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

fn all_extensions<'a>(filters: &'a [Filter<'a>]) -> Vec<&'a str> {
    filters.iter().flat_map(|x| x.extensions).copied().collect()
}

fn file_type_dropdown_items(filters: &[Filter<'_>]) -> Id<NSMutableArray<NSString>> {
    let mut titles = NSMutableArray::new();
    for filter in filters {
        let extensions: Vec<String> = filter
            .extensions
            .iter()
            .map(|s| format!("*.{}", s))
            .collect();
        let title = format!("{} ({})", filter.description, extensions.join(" "));
        titles.push(NSString::from_str(&title));
    }
    titles
}
