use gtk::{BoxExt, Entry, EntryExt, Orientation, WidgetExt};

pub fn add_resolution_box(vbox: &gtk::Box) -> (Entry, Entry) {
    let resolution_hbox = gtk::Box::new(Orientation::Horizontal, 5);
    let resolution_label = gtk::Label::new(Some("Resolution"));
    vbox.pack_start(&resolution_label, false, false, 0);

    resolution_hbox.set_halign(gtk::Align::Center);

    let width_entry = Entry::new();
    width_entry.set_placeholder_text(Some("1920"));
    resolution_hbox.pack_start(&width_entry, false, false, 0);

    let resolution_separator = gtk::Label::new(Some("x"));
    resolution_hbox.pack_start(&resolution_separator, false, false, 0);

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("1080"));
    resolution_hbox.pack_start(&height_entry, false, false, 0);
    vbox.pack_start(&resolution_hbox, false, false, 0);

    (width_entry, height_entry)
}
