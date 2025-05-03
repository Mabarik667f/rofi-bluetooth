use std::process::{Command, Output};

#[derive(Debug)]
struct ServiceAction<'a> {
    arg: &'a str,
    err_msg: &'a str,
}

trait BluetoothCtlRunner {
    fn run(&self) -> Output;
}

impl<'a> BluetoothCtlRunner for ServiceAction<'a> {
    fn run(&self) -> Output {
        Command::new("sh")
            .arg("-c")
            .arg(self.arg)
            .output()
            .expect(self.err_msg)
    }
}

pub fn toggle_power(power_off: bool) {
    let flag: &str;
    if power_off {
        let bl_locked = ServiceAction {
            arg: &format!("rfkill list bluetooth | grep -q 'blocked: yes'"),
            err_msg: "failed unblock bluetooth",
        }
        .run();
        if !bl_locked.stdout.is_empty() {
            ServiceAction {
                arg: &format!("rfkill unblock bluetooth | sleep 3"),
                err_msg: "failed unblock bluetooth",
            }
            .run();
        }
        flag = "on";
    } else {
        flag = "off";
    };
    ServiceAction {
        arg: format!("bluetoothctl power {flag}").as_str(),
        err_msg: "failed to toggle power",
    }
    .run();
}

pub fn toggle_scan(scan_on: bool) {
    if scan_on {
        ServiceAction {
            arg: &format!("kill $(pgrep -f 'bluetoothctl --timeout 5 scan on')"),
            err_msg: "failed to kill scan pids",
        }
        .run();
        ServiceAction {
            arg: &format!("bluetoothctl scan off"),
            err_msg: "failed to toggle scan",
        }
        .run();
    } else {
        let res = ServiceAction {
            arg: format!("bluetoothctl --timeout 5 scan on &").as_str(),
            err_msg: "failed to toggle scan",
        }
        .run();
        println!("{:?}", res);
    }
}

pub fn toggle_pairable(pairable_on: bool) {
    let flag = if pairable_on { "off" } else { "on" };

    ServiceAction {
        arg: &format!("bluetoothctl pairable {flag}"),
        err_msg: "failed to toggle scan",
    }
    .run();
}

pub fn toggle_discoverable(discoverable_on: bool) {
    let flag = if discoverable_on { "off" } else { "on" };

    ServiceAction {
        arg: &format!("bluetoothctl discoverable {flag}"),
        err_msg: "failed to toggle scan",
    }
    .run();
}

pub fn discoverable_on() -> bool {
    !check_option_on("Discoverable: yes")
}

pub fn scan_on() -> bool {
    !check_option_on("Discovering: yes")
}

pub fn pairable_on() -> bool {
    !check_option_on("Pairable: yes")
}

pub fn power_on() -> bool {
    check_option_on("Powered: yes")
}

fn check_option_on(option: &str) -> bool {
    let command_res = Command::new("sh")
        .arg("-c")
        .arg(format!("bluetoothctl show | grep '{option}'"))
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&command_res.stdout).is_empty()
}

pub fn get_devices() -> Vec<Device> {
    let res = Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl devices")
        .output()
        .expect("failed getting devices");
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
    pub fn trusted(&self) -> bool {
        self.check_option_on("Trusted: yes")
    }
    pub fn paired(&self) -> bool {
        self.check_option_on("Paired: yes")
    }
    pub fn connected(&self) -> bool {
        self.check_option_on("Connected: yes")
    }

    fn check_option_on(&self, option: &str) -> bool {
        let command_res = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "bluetoothctl info {0} | grep '{1}'",
                self.mac, option
            ))
            .output()
            .expect("failed to execute process");
        String::from_utf8_lossy(&command_res.stdout).is_empty()
    }

    pub fn toggle_trust(&self) {
        if !self.trusted() {
            ServiceAction {
                arg: &format!("bluetoothctl untrust {}", self.mac),
                err_msg: "failed to toggle scan",
            }
            .run();
        } else {
            ServiceAction {
                arg: &format!("bluetoothctl trust {}", self.mac),
                err_msg: "failed to toggle scan",
            }
            .run();
        }
    }
    pub fn toggle_connection(&self) {
        if !self.connected() {
            ServiceAction {
                arg: &format!("bluetoothctl disconnect {}", self.mac),
                err_msg: "failed to toggle scan",
            }
            .run();
        } else {
            ServiceAction {
                arg: &format!("bluetoothctl connect {}", self.mac),
                err_msg: "failed to toggle scan",
            }
            .run();
        }
    }
    pub fn toggle_paired(&self) {
        if !self.paired() {
            println!("RUN PAIRED REMOVE");
            ServiceAction {
                arg: &format!("bluetoothctl remove {}", self.mac),
                err_msg: "failed to toggle scan",
            }
            .run();
        } else {
            println!("RUN PAIRED PAIR");
            ServiceAction {
                arg: &format!("bluetoothctl pair {}", self.mac),
                err_msg: "failed to toggle scan",
            }
            .run();
        }
    }

    pub fn get_menu(&self) -> Vec<String> {
        let mut options: Vec<String> = vec![];

        let connected = self.connected();
        if connected {
            options.push("Connected: no".to_string());
        } else {
            options.push("Connected: yes".to_string());
        }

        let paired = self.paired();
        if paired {
            options.push("Paired: no".to_string());
        } else {
            options.push("Paired: yes".to_string());
        }

        let trusted = self.trusted();
        if trusted {
            options.push("Trusted: no".to_string());
        } else {
            options.push("Trusted: yes".to_string());
        }

        options.push("----------".to_string());
        options.push("Back".to_string());

        options
    }
}

mod tests {
    use super::*;
}
