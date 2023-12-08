use glib::signal::Inhibit;
use gtk::{Adjustment, Box, BoxExt, Scale, WidgetExt};

pub fn horizontal_scale(label: &str, adjustment: Adjustment, parent_box: &Box) -> Scale {
    let sample_size_label = gtk::Label::new(Some(label));
    parent_box.pack_start(&sample_size_label, false, false, 0);

    let sample_size_scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));

    sample_size_scale.set_hexpand(true);
    sample_size_scale.set_valign(gtk::Align::Start);
    sample_size_scale.connect_scroll_event(|_, _| {
        Inhibit(true) // This prevents the scale from being adjusted with the mouse scroll
    });
    parent_box.pack_start(&sample_size_scale, false, true, 0);
    sample_size_scale
}
