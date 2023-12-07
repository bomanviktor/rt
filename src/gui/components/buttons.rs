use gtk::{Box, BoxExt, Button, CssProvider, StyleContextExt, WidgetExt};

pub fn create_button_with_label(label: &str, provider: &CssProvider) -> Button {
    let button = Button::with_label(label);
    button
        .get_style_context()
        .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    button
}

pub fn create_buttons_row(
    hbox: &Box,
    provider: &CssProvider,
    btn_labels: [&str; 4],
) -> (Button, Button, Button, Button) {
    // Create a horizontal box for the side-by-side buttons

    hbox.set_halign(gtk::Align::Center);

    let mut buttons = [Button::new(), Button::new(), Button::new(), Button::new()];
    // Add buttons to the horizontal box
    for (i, label) in btn_labels.iter().enumerate() {
        let button = create_button_with_label(label, provider);
        hbox.pack_start(&button, false, false, 0);
        buttons[i] = button;
    }

    (
        buttons[0].clone(),
        buttons[1].clone(),
        buttons[2].clone(),
        buttons[3].clone(),
    )
}
