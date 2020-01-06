use gtk::prelude::*;

#[derive(Clone)]
pub struct Lightning {
    inner: gtk::Box,
    input: gtk::ComboBox,
    apply: gtk::Button
}

impl Lightning {

    pub fn build(builder: &gtk::Builder) -> Lightning {

        Lightning {
            inner: builder.get_object("lightning_box").expect("could not get lightning_box"),
            input: builder.get_object("lightning_input").expect("could not get lightning_input"),
            apply: builder.get_object("lightning_apply").expect("could not get lightning_apply")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

}
