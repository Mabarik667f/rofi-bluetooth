use crate::bluetooth::{
    controller::{
        get_menu_options, toggle_discoverable, toggle_pairable, toggle_power, toggle_scan,
    },
    device::{Device, get_devices},
};

pub enum BluetoothMenuResult {
    Continue,
    ExitWithError(String),
}

pub fn show_main_menu() -> BluetoothMenuResult {
    let devices = get_devices();
    let options = get_menu_options(&devices);

    match rofi::Rofi::new(&options).prompt("Bluetooth").run() {
        Ok(choice) => match choice.as_str() {
            "Power: on" | "Power: off" => {
                toggle_power();
                BluetoothMenuResult::Continue
            }
            "Scan: on" | "Scan: off" => {
                toggle_scan();
                BluetoothMenuResult::Continue
            }
            "Pairable: on" | "Pairable: off" => {
                toggle_pairable();
                BluetoothMenuResult::Continue
            }
            "Discoverable: on" | "Discoverable: off" => {
                toggle_discoverable();
                BluetoothMenuResult::Continue
            }
            _ => {
                let mut selected_device: Option<Device> = None;
                for device in devices {
                    if device.name == choice {
                        selected_device = Some(device);
                        break;
                    }
                }
                loop {
                    match show_device_menu(selected_device.as_ref()) {
                        DeviceMenuResult::ExitWithError(e) => {
                            return BluetoothMenuResult::ExitWithError(e);
                        }
                        DeviceMenuResult::Continue => {
                            continue;
                        }
                        DeviceMenuResult::Back => {
                            break;
                        }
                    }
                }
                println!("{:?}", choice);
                BluetoothMenuResult::Continue
            }
        },
        Err(rofi::Error::Interrupted) => {
            BluetoothMenuResult::ExitWithError("Interrupted".to_string())
        }
        Err(e) => BluetoothMenuResult::ExitWithError(e.to_string()),
    }
}

pub enum DeviceMenuResult {
    Back,
    Continue,
    ExitWithError(String),
}

pub fn show_device_menu(device: Option<&Device>) -> DeviceMenuResult {
    match device {
        Some(device) => {
            let device_options: Vec<String> = device.get_menu();
            match rofi::Rofi::new(&device_options).prompt(&device.name).run() {
                Ok(dev) => match dev.as_str() {
                    "Trusted: off" | "Trusted: on" => {
                        device.toggle_trust();
                        DeviceMenuResult::Continue
                    }
                    "Paired: off" | "Paired: on" => {
                        device.toggle_paired();
                        DeviceMenuResult::Continue
                    }
                    "Connected: off" | "Connected: on" => {
                        device.toggle_connection();
                        DeviceMenuResult::Continue
                    }
                    "Back" => DeviceMenuResult::Back,
                    _ => {
                        println!("{:?}", dev);
                        DeviceMenuResult::Continue
                    }
                },
                Err(rofi::Error::Interrupted) => {
                    DeviceMenuResult::ExitWithError("Interrupted".to_string())
                }
                Err(e) => DeviceMenuResult::ExitWithError(e.to_string()),
            }
        }
        None => DeviceMenuResult::Back,
    }
}
