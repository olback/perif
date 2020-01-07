use gtk::prelude::*;

pub fn build(builder: &gtk::Builder) -> gtk::AboutDialog {

    let dialog: gtk::AboutDialog = builder.get_object("about_dialog").expect("could not get about_dialog");

    dialog.set_version(Some(env!("CARGO_PKG_VERSION")));

    dialog

}
