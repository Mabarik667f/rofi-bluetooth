use super::{BluetoothCtlRunner, ServiceAction, format_option};

pub fn get_devices() -> Vec<Device> {
    let res = ServiceAction {
        arg: "bluetoothctl devices",
        err_msg: "failed getting devices",
    }
    .run();

    let mut devices: Vec<Device> = vec![];
    for device in String::from_utf8_lossy(&res.stdout).split("\n") {
        if device.is_empty() {
            continue;
        };

        let device_sp = device.split(" ").collect::<Vec<&str>>();

        devices.push(Device {
            mac: device_sp[1].to_string(),
            name: device_sp[2..].join(" "),
        });
    }
    devices
}

#[derive(Debug)]
pub struct Device {
    pub mac: String,
    pub name: String,
}

impl Device {
    pub fn get_menu(&self) -> Vec<String> {
        let mut options: Vec<String> = vec![];

        options.push(format_option("Connected", || self.connected()));
        options.push(format_option("Paired", || self.paired()));
        options.push(format_option("Trusted", || self.trusted()));

        options.push("----------".to_string());
        options.push("Back".to_string());

        options
    }

    fn trusted(&self) -> bool {
        self.check_option_on("Trusted: yes")
    }

    fn paired(&self) -> bool {
        self.check_option_on("Paired: yes")
    }

    fn connected(&self) -> bool {
        self.check_option_on("Connected: yes")
    }

    fn check_option_on(&self, option: &str) -> bool {
        let cmd_res = ServiceAction {
            arg: &format!("bluetoothctl info {0} | grep '{1}'", self.mac, option),
            err_msg: "failed to execute process",
        }
        .run();
        !String::from_utf8_lossy(&cmd_res.stdout).is_empty()
    }

    pub fn toggle_trust(&self) {
        let err_msg = "failed to toggle trust";
        let arg = if self.trusted() {
            &format!("bluetoothctl untrust {}", self.mac)
        } else {
            &format!("bluetoothctl trust {}", self.mac)
        };

        ServiceAction { arg, err_msg }.run();
    }

    pub fn toggle_connection(&self) {
        let err_msg = "failed to toggle connect";
        let arg = if self.connected() {
            &format!("bluetoothctl disconnect {}", self.mac)
        } else {
            &format!("bluetoothctl connect {}", self.mac)
        };

        ServiceAction { arg, err_msg }.run();
    }

    pub fn toggle_paired(&self) {
        let err_msg = "failed to toggle pair";
        let arg = if self.paired() {
            &format!("bluetoothctl remove {}", self.mac)
        } else {
            &format!("bluetoothctl pair {}", self.mac)
        };

        ServiceAction { arg, err_msg }.run();
    }
}
