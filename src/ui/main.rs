use gtk::{
    prelude::*,
    Application,
    ApplicationWindow,
    Builder
};
use crate::get_obj;

pub fn build(builder: &Builder, app: &Application) -> ApplicationWindow {

    let window: ApplicationWindow = get_obj!(builder, "main_window");
    window.set_application(Some(app));
    window.show_all();

    window

}
