use gtk::prelude::*;
use gio::{SettingsExt, SettingsBindFlags};
use super::Ui;
use crate::get_obj;

pub struct Settings;

impl Settings {

    pub fn connect(builder: &gtk::Builder, ui: &Ui, settings: gio::Settings) {

        // Connect back button
        let back_button: gtk::Button = get_obj!(builder, "settings_back");
        let stack_clone = ui.stack.clone();
        back_button.connect_clicked(move |_| {
            stack_clone.hide_settings(true);
        });

        // Connect settings
        let s_show_information: gtk::Switch = get_obj!(builder, "settings_show_information");
        settings.bind("show-information", &s_show_information, "active", SettingsBindFlags::DEFAULT);

        let s_show_command_result: gtk::Switch = get_obj!(builder, "settings_show_command_result");
        settings.bind("show-command-result", &s_show_command_result, "active", SettingsBindFlags::DEFAULT);


        let s_notify_at: gtk::SpinButton = get_obj!(builder, "settings_notify_at");
        settings.bind("notify-at", &s_notify_at, "value", SettingsBindFlags::DEFAULT);

        let s_notify_interval: gtk::SpinButton = get_obj!(builder, "settings_notify_interval");
        settings.bind("notify-interval", &s_notify_interval, "value", SettingsBindFlags::DEFAULT);

        // Connect reset button
        let settings_reset: gtk::Button = get_obj!(builder, "settings_reset");
        settings_reset.connect_clicked(move |_| {
            let keys = settings.list_keys();
            for key in keys {
                settings.reset(key.as_str());
            }
        });

    }

}
