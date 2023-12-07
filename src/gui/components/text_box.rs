use gtk::{
    Box, BoxExt, CssProvider, Entry, EntryExt, Grid, GridExt, Label, LabelBuilder, StyleContextExt,
    WidgetExt,
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
    vbox: &Box,
    label_text: &str,
    placeholders: [&str; 3],
) -> (Entry, Entry, Entry) {
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
        let top_label = i as i32 + 1;
        let top_entry = i as i32 + 2;
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
