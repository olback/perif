use gtk::prelude::*;
use crate::get_obj;

pub fn build(builder: &gtk::Builder) -> gtk::AboutDialog {

    let dialog: gtk::AboutDialog = get_obj!(builder, "about_dialog");

    dialog.set_version(Some(include_str!("../../out/version.txt")));

    dialog

}
