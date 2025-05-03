use rofi;
use rofi_bluetooth::{
    Device, discoverable_on, format_option, get_devices, pairable_on, power_on, scan_on,
    toggle_discoverable, toggle_pairable, toggle_power, toggle_scan,
};

fn main() {
    let mut device_options: Vec<String> = vec![];
    let mut selected_device: Option<Device> = None;
    loop {
        let mut options: Vec<String>;
        let mut devices: Vec<Device> = vec![];

        if !power_on() {
            options = vec!["Power: off".to_string()];
        } else {
            options = vec![];
            devices = get_devices();
            for device in &devices {
                options.push(device.name.clone());
            }

            options.push("----------".to_string());
            options.push("Power: on".to_string());

            options.push(format_option("Scan", scan_on));
            options.push(format_option("Discoverable", discoverable_on));
            options.push(format_option("Pairable", pairable_on));
        }

        let mut flag = false;

        match &selected_device {
            Some(device) => {
                flag = true;
                match rofi::Rofi::new(&device_options).prompt(&device.name).run() {
                    Ok(dev) => match dev.as_str() {
                        "Trusted: off" | "Trusted: on" => device.toggle_trust(),
                        "Paired: off" | "Paired: on" => device.toggle_paired(),
                        "Connected: off" | "Connected: on" => device.toggle_connection(),
                        "Back" => flag = false,
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
                    "Power: on" | "Power: off" => toggle_power(),
                    "Scan: on" | "Scan: off" => toggle_scan(),
                    "Pairable: on" | "Pairable: off" => toggle_pairable(),
                    "Discoverable: on" | "Discoverable: off" => toggle_discoverable(),
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
