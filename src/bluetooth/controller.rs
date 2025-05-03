use super::{BluetoothCtlRunner, ServiceAction, device::Device, format_option};

pub fn get_menu_options(devices: &Vec<Device>) -> Vec<String> {
    if !power_on() {
        return vec!["Power: off".to_string()];
    }
    let mut options = vec![];
    options.extend(devices.iter().map(|d| d.name.clone()));

    options.push("----------".to_string());
    options.push("Power: on".to_string());

    options.push(format_option("Scan", scan_on));
    options.push(format_option("Discoverable", discoverable_on));
    options.push(format_option("Pairable", pairable_on));

    options
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
