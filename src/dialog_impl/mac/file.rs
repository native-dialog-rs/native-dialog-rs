use super::ffi::cocoa::{
    INSColor, INSOpenPanel, INSPopUpButton, INSSavePanel, INSStackView, INSTextField, NSColor,
    NSEdgeInsets, NSOpenPanel, NSPopUpButton, NSSavePanel, NSStackView, NSStackViewGravity,
    NSTextField, NSUserInterfaceLayoutOrientation, INSURL,
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

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_file_types(get_allowed_types(&self.filters));

        match panel.run_modal() {
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

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_file_types(get_types_dropdown_items(&self.filters));

        match panel.run_modal() {
            Ok(urls) => Ok(urls.to_vec().into_iter().map(INSURL::to_path_buf).collect()),
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

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        match panel.run_modal() {
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

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        // If there are filters specified, show a dropdown on the panel
        if let Some(first_filter) = self.filters.first() {
            let action_target = DropdownAction::new();
            action_target.set_save_panel(panel.clone());
            action_target.set_filters(&self.filters as *const Vec<_> as usize);

            let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(480.0, 0.0));
            let dropdown = NSPopUpButton::new_with_frame(frame, false);
            dropdown.add_items_with_titles(get_types_dropdown_items(&self.filters));
            dropdown.select_item_at(0);
            dropdown.set_action(sel!(onItemSelected:));
            dropdown.set_target(action_target);

            let label = NSTextField::new_label_with_string("File Type: ");
            label.set_text_color(NSColor::secondary_label_color());

            let stack = NSStackView::new();
            // Edge insets in specific axis are only enforced when hugging priority >= 500
            // See https://stackoverflow.com/questions/54533509/nsstackview-edgeinsets-gets-ignored
            stack.set_hugging_priority(500.0, NSUserInterfaceLayoutOrientation::Vertical);
            stack.set_edge_insets(NSEdgeInsets::new(16.0, 0.0, 16.0, 0.0));
            stack.add_view_in_gravity(label, NSStackViewGravity::Center);
            stack.add_view_in_gravity(dropdown, NSStackViewGravity::Center);

            panel.set_allowed_file_types(first_filter.extensions);
            panel.set_accessory_view(stack);
        }

        match panel.run_modal() {
            Ok(url) => Ok(Some(url.to_path_buf())),
            Err(_) => Ok(None),
        }
    }
}

fn get_allowed_types(filters: &[Filter<'_>]) -> Id<NSMutableArray<NSString>> {
    let mut extensions = NSMutableArray::new();
    for filter in filters {
        for ext in filter.extensions {
            let s = NSString::from_str(ext);
            extensions.add_object(s);
        }
    }
    extensions
}

fn get_types_dropdown_items(filters: &[Filter<'_>]) -> Id<NSMutableArray<NSString>> {
    let mut titles = NSMutableArray::new();
    for filter in filters {
        let extensions: Vec<String> = filter
            .extensions
            .iter()
            .map(|s| format!("*.{}", s))
            .collect();
        let title = format!("{} ({})", filter.description, extensions.join(", "));
        titles.add_object(NSString::from_str(&title));
    }
    titles
}
