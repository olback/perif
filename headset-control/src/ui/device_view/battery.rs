use gtk::prelude::*;
use libhc::BatteryState;

#[derive(Clone)]
pub struct Battery {
    inner: gtk::Box,
    label: gtk::Label,
}

impl Battery {

    pub fn build(builder: &gtk::Builder) -> Battery {

        Battery {
            inner: builder.get_object("battery_box").expect("could not get battery_box"),
            label: builder.get_object("battery_label").expect("could not get battery_label"),
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

    pub fn set_battery(&self, battery: BatteryState) {

        match battery {
            BatteryState::Charging(level) => {
                match level {
                    Some(l) => self.label.set_text(&format!("Charging ({}%)", l)),
                    None => self.label.set_text("Charging")
                }
            },
            BatteryState::Discharging(level) => self.label.set_text(&format!("{}%", level)),
            BatteryState::Full => self.label.set_text("Full"),
            BatteryState::Unavailable => self.label.set_text("Unavailable")
        }

    }

}
