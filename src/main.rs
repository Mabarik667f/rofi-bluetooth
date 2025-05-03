use rofi;
use rofi_bluetooth::{
    Device, discoverable_on, get_devices, pairable_on, power_on, scan_on, toggle_discoverable,
    toggle_pairable, toggle_power, toggle_scan,
};

fn main() {
    let mut device_options: Vec<String> = vec![];
    let mut selected_device: Option<Device> = None;
    loop {
        let power_off = power_on();

        let mut options: Vec<String>;
        let mut devices: Vec<Device> = vec![];
        let mut scan: bool = false;
        let mut discoverable: bool = false;
        let mut pairable: bool = false;

        if power_off {
            options = vec!["Power: no".to_string()];
        } else {
            options = vec![];
            devices = get_devices();
            for device in &devices {
                options.push(device.name.clone());
            }

            options.push("----------".to_string());
            options.push("Power: yes".to_string());

            scan = scan_on();
            if scan {
                options.push("Scan: on".to_string());
            } else {
                options.push("Scan: off".to_string());
            }

            discoverable = discoverable_on();
            if discoverable {
                options.push("Discoverable: on".to_string());
            } else {
                options.push("Discoverable: off".to_string());
            }

            pairable = pairable_on();
            if pairable {
                options.push("Pairable: on".to_string());
            } else {
                options.push("Pairable: off".to_string());
            }
        }

        let mut flag = false;

        match &selected_device {
            Some(device) => {
                flag = true;
                match rofi::Rofi::new(&device_options).prompt(&device.name).run() {
                    Ok(dev) => match dev.as_str() {
                        "Trusted: no" | "Trusted: yes" => device.toggle_trust(),
                        "Paired: no" | "Paired: yes" => device.toggle_paired(),
                        "Connected: no" | "Connected: yes" => device.toggle_connection(),
                        _ => println!("{:?}", dev),
                    },
                    Err(rofi::Error::Interrupted) => {
                        println!("Interrupted");
                        break;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
            None => println!("NNOOOONE"),
        }

        if !flag {
            match rofi::Rofi::new(&options).prompt("Bluetooth").run() {
                Ok(choice) => match choice.as_str() {
                    "Power: no" | "Power: yes" => toggle_power(power_off),
                    "Scan: on" | "Scan: off" => toggle_scan(scan),
                    "Pairable: on" | "Pairable: off" => toggle_pairable(pairable),
                    "Discoverable: on" | "Discoverable: off" => toggle_discoverable(discoverable),
                    _ => {
                        for device in devices {
                            if device.name == choice {
                                device_options = device.get_menu();
                                selected_device = Some(device);
                            }
                        }
                        println!("{:?}", choice)
                    }
                },
                Err(rofi::Error::Interrupted) => {
                    println!("Interrupted");
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }
}
