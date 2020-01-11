use gtk::prelude::*;
use libperif::BatteryState;
use crate::get_obj;

#[derive(Clone)]
pub struct Battery {
    inner: gtk::Box,
    label: gtk::Label,
}

impl Battery {

    pub fn build(builder: &gtk::Builder) -> Battery {

        Battery {
            inner: get_obj!(builder, "battery_box"),
            label: get_obj!(builder, "battery_label")
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

    pub fn set_battery(&self, battery: BatteryState) {

        match battery {
            BatteryState::Charging(level) => match level {
                Some(l) => self.label.set_text(&format!("Charging ({}%)", l)),
                None => self.label.set_text("Charging")
            },
            BatteryState::Discharging(level) => self.label.set_text(&format!("{}%", level)),
            BatteryState::Full => self.label.set_text("Full"),
            BatteryState::Unavailable => self.label.set_text("Unavailable"),
            BatteryState::Error(err) => self.label.set_text(err.as_str())
        }

    }

}
