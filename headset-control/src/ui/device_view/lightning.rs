use gtk::prelude::*;

#[derive(Clone)]
pub struct Lightning {
    inner: gtk::Box,
    on: gtk::Button,
    off: gtk::Button
}

impl Lightning {

    pub fn build(builder: &gtk::Builder) -> Lightning {

        Lightning {
            inner: builder.get_object("lightning_box").expect("could not get lightning_box"),
            on: builder.get_object("lightning_on").expect("could not get lightning_on"),
            off: builder.get_object("lightning_off").expect("could not get lightning_off")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

}
