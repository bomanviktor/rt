use glib::clone;
use gtk::WidgetExt;
use gtk::{AboutDialog, AboutDialogExt, Button, ButtonExt, GtkWindowExt};
use gtk::{CssProvider, DialogExt, StyleContextExt};

pub fn about_dialog(scaled_icon: &gdk_pixbuf::Pixbuf) -> AboutDialog {
    // About Dialog window
    let about_dialog = AboutDialog::new();
    about_dialog.set_program_name("Grit:Lab Ray Tracing Project");
    about_dialog.set_comments(Some(
        "Completed during grit:lab full-stack development course as part of the Rust Studies. \n
        December 2023",
    ));
    about_dialog.set_authors(&[
        "Viktor Boman",
        "Johannes Eckerman",
        "Salam Foon",
        "Ville Patjas",
        "André Teetor",
    ]);
    about_dialog.set_website_label(Some("Code Repository"));
    about_dialog.set_website(Some("https://github.com/bomanviktor/rt"));
    about_dialog.set_logo(Some(scaled_icon));
    about_dialog.set_modal(true);
    about_dialog.set_destroy_with_parent(true);

    about_dialog
}

pub fn about_btn(about_dialog: &AboutDialog, provider: &CssProvider) -> Button {
    let about_button = Button::with_label("About");
    about_button.connect_clicked(clone!(@weak about_dialog => move |_| {
        about_dialog.run();
        about_dialog.hide();
    }));
    about_button
        .get_style_context()
        .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    about_button
}
