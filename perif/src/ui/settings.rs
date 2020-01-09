use gtk::prelude::*;
// use gio::{SettingsExt, SettingsBindFlags};
use super::Ui;

pub struct Settings;

impl Settings {

    pub fn connect(builder: &gtk::Builder, ui: &Ui/*, settings: gio::Settings*/) {

        // Connect back button
        let back_button: gtk::Button = builder.get_object("settings_back").expect("could not get settings_back");
        let stack_clone = ui.stack.clone();
        back_button.connect_clicked(move |_| {
            stack_clone.hide_settings(true);
        });

        // Connect settings

        // Connect reset button
        let settings_reset: gtk::Button = builder.get_object("settings_reset").expect("could not get settings_reset");
        settings_reset.connect_clicked(move |_| {
            // let keys = settings.list_keys();
            // for key in keys {
            //     settings.reset(key.as_str());
            // }
        });

    }

}
