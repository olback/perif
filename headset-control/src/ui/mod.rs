use gtk::{
    prelude::*,
    Application,
    Builder
};

mod main;

pub struct Ui {
    main: gtk::ApplicationWindow
}

impl Ui {

    pub fn build(app: &Application) -> Self {

        let builder = Builder::new_from_resource("/net/olback/headset-control/ui");

        Self {
            main: main::build(&builder, &app)
        }

    }

}
