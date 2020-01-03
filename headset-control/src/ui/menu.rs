use gtk::prelude::*;
use super::Ui;

pub struct Menu;

impl Menu {

    pub fn connect(builder: &gtk::Builder, ui: &Ui) {

        let settings_button: gtk::Button = builder.get_object("open_about").expect("could not get open_about");
        let about_clone = ui.about_dialog.clone();
        settings_button.connect_clicked(move |_| {
            match about_clone.run() {
                _ => about_clone.hide()
            }
        });


        let about_button: gtk::Button = builder.get_object("open_settings").expect("could not get open_settings");
        let stack_clone = ui.stack.clone();
        about_button.connect_clicked(move |_| {
            let mut stack_clone_2 = stack_clone.clone();
            stack_clone_2.show_settings(true);
        });

    }

}
