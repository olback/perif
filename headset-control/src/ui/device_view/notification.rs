use gtk::prelude::*;

#[derive(Clone)]
pub struct Notification {
    inner: gtk::Box,
    sound1: gtk::Button,
    sound2: gtk::Button
}

impl Notification {

    pub fn build(builder: &gtk::Builder) -> Notification {

        Notification {
            inner: builder.get_object("notification_box").expect("could not get notification_box"),
            sound1: builder.get_object("notification_sound1").expect("could not get notification_sound1"),
            sound2: builder.get_object("notification_sound2").expect("could not get notification_sound2")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

}
