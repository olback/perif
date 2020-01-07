use gtk::prelude::*;
use super::Ui;

pub struct Settings;

impl Settings {

    pub fn connect(builder: &gtk::Builder, ui: &Ui) {

        let back_button: gtk::Button = builder.get_object("settings_back").expect("could not get settings_back");
        let stack_clone = ui.stack.clone();
        back_button.connect_clicked(move |_| {
            stack_clone.hide_settings(true);
        });

        // TODO: Add settings connect stuff!

    }

}
