use gtk::prelude::*;
use gio::{SettingsExt, SettingsBindFlags};
use super::Ui;

pub struct Settings;

impl Settings {

    pub fn connect(builder: &gtk::Builder, ui: &Ui, settings: gio::Settings) {

        // Connect back button
        let back_button: gtk::Button = builder.get_object("settings_back").expect("could not get settings_back");
        let stack_clone = ui.stack.clone();
        back_button.connect_clicked(move |_| {
            stack_clone.hide_settings(true);
        });

        // Connect settings
        let s_show_information: gtk::Switch = builder.get_object("settings_show_information").expect("could not get settings_show_information");
        settings.bind("show-information", &s_show_information, "active", SettingsBindFlags::DEFAULT);

        let s_show_command_result: gtk::Switch = builder.get_object("settings_show_command_result").expect("could not get settings_show_command_result");
        settings.bind("show-command-result", &s_show_command_result, "active", SettingsBindFlags::DEFAULT);


        // Connect reset button
        let settings_reset: gtk::Button = builder.get_object("settings_reset").expect("could not get settings_reset");
        settings_reset.connect_clicked(move |_| {
            let keys = settings.list_keys();
            for key in keys {
                settings.reset(key.as_str());
            }
        });

    }

}
