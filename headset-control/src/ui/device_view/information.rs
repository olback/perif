use gtk::prelude::*;

#[derive(Clone)]
pub struct Information {
    inner: gtk::Box,
    label: gtk::Label
}

impl Information {

    pub fn build(builder: &gtk::Builder) -> Information {

        Information {
            inner: builder.get_object("information_box").expect("could not find information_box"),
            label: builder.get_object("information_label").expect("could not find information_label")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

    pub fn set_text<T: Into<String>>(&self, text: T) {

        self.label.set_text(&text.into());

    }

}
