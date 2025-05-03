use rofi;
use rofi_bluetooth::{
    discoverable_on, get_devices, pairable_on, power_on, scan_on, toggle_discoverable,
    toggle_pairable, toggle_power, toggle_scan,
};

fn main() {
    loop {
        let power_off = power_on();

        let mut options: Vec<String>;
        let mut scan: bool = false;
        let mut discoverable: bool = false;
        let mut pairable: bool = false;

        if power_off {
            options = vec!["Power: no".to_string()];
        } else {
            options = vec![];
            let devices = get_devices();
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
        match rofi::Rofi::new(&options).prompt("Bluetooth").run() {
            Ok(choice) => match choice.as_str() {
                "Power: no" | "Power: yes" => toggle_power(power_off),
                "Scan: on" | "Scan: off" => toggle_scan(scan),
                "Pairable: on" | "Pairable: off" => toggle_pairable(pairable),
                "Discoverable: on" | "Discoverable: off" => toggle_discoverable(discoverable),
                _ => println!("{:?}", choice),
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
