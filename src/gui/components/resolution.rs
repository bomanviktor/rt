use gtk::{BoxExt, Entry, EntryExt, FlowBox, WidgetExt};

pub fn add_resolution_box(
    vbox: &gtk::Box,
    flow_box: &FlowBox,
    resolution_hbox: &gtk::Box,
) -> (Entry, Entry) {
    let resolution_label = gtk::Label::new(Some("Resolution"));
    vbox.pack_start(&resolution_label, false, false, 0);

    resolution_hbox.set_halign(gtk::Align::Center);

    let width_entry = Entry::new();
    width_entry.set_placeholder_text(Some("Width Default: 800"));
    resolution_hbox.pack_start(&width_entry, false, false, 0);

    let resolution_separator = gtk::Label::new(Some("x"));
    resolution_hbox.pack_start(&resolution_separator, false, false, 0);

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("Height Default: 600"));
    resolution_hbox.pack_start(&height_entry, false, false, 0);
    vbox.pack_start(resolution_hbox, false, false, 0);
    vbox.pack_start(flow_box, false, false, 0);

    (width_entry, height_entry)
}
