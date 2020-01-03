use gtk::prelude::*;

#[derive(Clone)]
pub struct Stack {
    inner: gtk::Stack,
    transition: gtk::StackTransitionType
}

impl Stack {

    pub fn build(builder: &gtk::Builder) -> Stack {

        let stack: gtk::Stack = builder.get_object("views").expect("could not find views");

        Stack {
            transition: stack.get_transition_type(),
            inner: stack
        }

    }

    pub fn show_devices(&self, transition: bool) {

        self.enable_transition(transition);
        self.inner.set_visible_child_name("devices");

    }

    pub fn show_no_devices(&self, transition: bool) {

        self.enable_transition(transition);
        self.inner.set_visible_child_name("no_devices");

    }

    pub fn show_settings(&mut self, transition: bool) {

        self.enable_transition(transition);
        self.inner.set_visible_child_name("settings");

    }

    pub fn hide_settings(&self, transition: bool) {

        self.enable_transition(transition);
        self.inner.set_visible_child_name("devices");

    }

    pub fn get_view_name(&self) -> String {

        self.inner.get_visible_child_name().unwrap().to_string()

    }

    fn enable_transition(&self, enable: bool) {

        if enable {
            self.inner.set_transition_type(self.transition);
        } else {
            self.inner.set_transition_type(gtk::StackTransitionType::None);
        }

    }

}
