use gtk::prelude::*;
use crate::get_obj;

#[derive(Clone)]
pub struct Information {
    inner: gtk::Box,
    label: gtk::Label
}

impl Information {

    pub fn build(builder: &gtk::Builder) -> Information {

        Information {
            inner: get_obj!(builder, "information_box"),
            label: get_obj!(builder, "information_label")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

    pub fn set_text<T: Into<String>>(&self, text: T) {

        self.label.set_text(&text.into());

    }

}
