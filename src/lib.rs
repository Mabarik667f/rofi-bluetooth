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

pub fn format_option<F>(option: &str, f: F) -> String
where
    F: Fn() -> bool,
{
    format!("{}: {}", option, if f() { "on" } else { "off" }).to_string()
}

fn unblock_bluetooth() {
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
}

pub fn toggle_power() {
    let command = if power_on() {
        "off"
    } else {
        unblock_bluetooth();
        "on"
    };

    ServiceAction {
        arg: &format!("bluetoothctl power {command}"),
        err_msg: "failed to toggle power",
    }
    .run();
}

fn kill_scan() {
    ServiceAction {
        arg: &format!("kill $(pgrep -f 'bluetoothctl --timeout 5 scan on')"),
        err_msg: "failed to kill scan pids",
    }
    .run();
}

pub fn toggle_scan() {
    let err_msg = "failed to toggle scan";
    let arg = if scan_on() {
        kill_scan();
        &format!("bluetoothctl scan off")
    } else {
        &format!("bluetoothctl --timeout 5 scan on &")
    };

    ServiceAction { arg, err_msg }.run();
}

pub fn toggle_pairable() {
    let command = if pairable_on() { "off" } else { "on" };

    ServiceAction {
        arg: &format!("bluetoothctl pairable {command}"),
        err_msg: "failed to toggle scan",
    }
    .run();
}

pub fn toggle_discoverable() {
    let command = if discoverable_on() { "off" } else { "on" };

    ServiceAction {
        arg: &format!("bluetoothctl discoverable {command}"),
        err_msg: "failed to toggle scan",
    }
    .run();
}

pub fn discoverable_on() -> bool {
    check_option_on("Discoverable: yes")
}

pub fn scan_on() -> bool {
    check_option_on("Discovering: yes")
}

pub fn pairable_on() -> bool {
    check_option_on("Pairable: yes")
}

pub fn power_on() -> bool {
    check_option_on("Powered: yes")
}

fn check_option_on(option: &str) -> bool {
    let cmd_res = ServiceAction {
        arg: &format!("bluetoothctl show | grep '{option}'"),
        err_msg: "failed to execute process",
    }
    .run();
    !String::from_utf8_lossy(&cmd_res.stdout).is_empty()
}

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
