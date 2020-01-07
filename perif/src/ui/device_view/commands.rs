use gtk::prelude::*;

#[derive(Clone)]
pub struct Commands {
    inner: gtk::Box,
    combo_box: gtk::ComboBox,
    run: gtk::Button
}

impl Commands {

    pub fn build(builder: &gtk::Builder) -> Commands {

        Commands {
            inner: builder.get_object("commands_box").expect("could not get commands_box"),
            combo_box: builder.get_object("commands_input").expect("could not get commands_input"),
            run: builder.get_object("commands_run").expect("could not get commands_run")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

}
