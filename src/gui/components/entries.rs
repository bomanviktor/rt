use gtk::{
    Box, BoxExt, CssProvider, Entry, EntryExt, Grid, GridExt, Label, LabelBuilder, Orientation,
    StyleContextExt, WidgetExt,
};

pub fn create_label_with_placeholder(text: &str) -> Label {
    let label = LabelBuilder::new().label(text).build();
    label.set_halign(gtk::Align::Start);
    label.set_valign(gtk::Align::Start);
    label
}

pub fn create_entry_with_placeholder(placeholder: &str) -> Entry {
    let entry = Entry::new();
    entry.set_placeholder_text(Some(placeholder));
    entry
}

pub fn add_coordinate_widgets_box(
    horizontal_box: &Box,
    label_text: &str,
    placeholders: [&str; 3],
) -> (Entry, Entry, Entry) {
    let vbox = Box::new(Orientation::Vertical, 0);
    let label = Label::new(Some(label_text));
    vbox.pack_start(&label, false, false, 0);

    let mut entries: [Entry; 3] = [Entry::new(), Entry::new(), Entry::new()];
    for (i, placeholder) in placeholders.iter().enumerate() {
        let label_text = match i {
            0 => "X pos",
            1 => "Y pos",
            _ => "Z pos",
        };
        let label = create_label_with_placeholder(label_text);
        vbox.pack_start(&label, false, false, 0);

        let entry = create_entry_with_placeholder(placeholder);
        vbox.pack_start(&entry, false, false, 0);
        entries[i] = entry;
    }

    horizontal_box.pack_start(&vbox, true, true, 0);

    (entries[0].clone(), entries[1].clone(), entries[2].clone())
}

pub fn add_coordinate_widgets_grid(
    grid: &Grid,
    provider: &CssProvider,
    label_text: &str,
    placeholders: &[&str],
) -> Vec<Entry> {
    let label = Label::new(Some(label_text));
    grid.attach(&label, 0, 0, 1, 1);

    let mut entries = Vec::new();
    for (i, placeholder) in placeholders.iter().enumerate() {
        let top_label = i as i32 * 2 + 1;
        let top_entry = top_label + 1;
        let label_text = match i {
            0 => "X pos",
            1 => "Y pos",
            2 => "Z pos",
            3 => {
                if label_text == "Cube" {
                    "Size"
                } else {
                    "Radius"
                }
            }
            _ => "Height",
        };

        let label = Label::new(Some(label_text));
        grid.attach(&label, 0, top_label, 1, 1);

        let entry = create_entry_with_placeholder(placeholder);
        grid.attach(&entry, 0, top_entry, 1, 1);
        let style_context = entry.get_style_context();
        style_context.add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        entries.push(entry);
    }
    entries
}

pub fn add_resolution_box(vbox: &Box) -> (Entry, Entry) {
    let resolution_hbox = Box::new(Orientation::Horizontal, 5);
    let resolution_label = Label::new(Some("Resolution"));
    vbox.pack_start(&resolution_label, false, false, 0);

    resolution_hbox.set_halign(gtk::Align::Center);

    let width_entry = Entry::new();
    width_entry.set_placeholder_text(Some("Width"));
    resolution_hbox.pack_start(&width_entry, false, false, 0);

    let resolution_separator = Label::new(Some("x"));
    resolution_hbox.pack_start(&resolution_separator, false, false, 0);

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("Height"));
    resolution_hbox.pack_start(&height_entry, false, false, 0);
    vbox.pack_start(&resolution_hbox, false, false, 0);

    (width_entry, height_entry)
}
