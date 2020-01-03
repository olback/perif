use gtk::prelude::*;

#[derive(Clone)]
pub struct Sidetone {
    inner: gtk::Box,
    scale: gtk::Scale,
    button: gtk::Button
}

impl Sidetone {

    pub fn build(builder: &gtk::Builder) -> Sidetone {

        Sidetone {
            inner: builder.get_object("sidetone_box").expect("could not get sidetone_box"),
            scale: builder.get_object("sidetone_scale").expect("could not get sidetone_scale"),
            button: builder.get_object("sidetone_apply").expect("could not get sidetone_apply")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

}
