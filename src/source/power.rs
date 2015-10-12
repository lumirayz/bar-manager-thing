extern crate dbus;

use self::dbus::MessageItem;

#[derive(Clone, Copy, Debug)]
pub enum BatteryStatus {
    Unknown,
    Charging,
    Discharging,
    Empty,
    Full,
    PendingCharge,
    PendingDischarge
}

pub struct PowerSource {
    conn: dbus::Connection
}

impl PowerSource {
    pub fn new() -> PowerSource {
        let c = dbus::Connection::get_private(dbus::BusType::System).unwrap();
        PowerSource { conn: c }
    }

    pub fn get(&self) -> (f64, BatteryStatus) {
        let p = dbus::Props::new(&self.conn,
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower/devices/battery_BAT0",
            "org.freedesktop.UPower.Device", 10000);
        let bat = p.get("Percentage").unwrap();
        let pct = match bat {
            MessageItem::Double(pct) => pct,
            _ => 0.0
        };
        let status = p.get("State").unwrap();
        let state = match status {
            MessageItem::UInt32(n) => match n {
                0 => BatteryStatus::Unknown,
                1 => BatteryStatus::Charging,
                2 => BatteryStatus::Discharging,
                3 => BatteryStatus::Empty,
                4 => BatteryStatus::Full,
                5 => BatteryStatus::PendingCharge,
                6 => BatteryStatus::PendingDischarge,
                _ => BatteryStatus::Unknown
            },
            _ => BatteryStatus::Unknown
        };
        (pct, state)
    }
}
