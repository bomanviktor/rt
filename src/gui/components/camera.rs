use gtk::{Box, BoxExt, Entry, EntryExt, Label, LabelBuilder, WidgetExt};

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

pub fn add_coordinate_widgets(
    vbox: &Box,
    label_text: &str,
    placeholders: [&str; 3],
) -> (Entry, Entry, Entry) {
    let camera_label = Label::new(Some(label_text));
    vbox.pack_start(&camera_label, false, false, 0);

    let mut entries: [Entry; 3] = [Entry::new(), Entry::new(), Entry::new()];
    for (i, placeholder) in placeholders.iter().enumerate() {
        let label_text = match i {
            0 => "X",
            1 => "Y",
            _ => "Z",
        };
        let label = create_label_with_placeholder(label_text);
        vbox.pack_start(&label, false, false, 0);

        let entry = create_entry_with_placeholder(placeholder);
        vbox.pack_start(&entry, false, false, 0);
        entries[i] = entry;
    }

    (entries[0].clone(), entries[1].clone(), entries[2].clone())
}
