use rofi;
use rofi_bluetooth::toggle_power;
use std::process::Command;

fn main() {
    loop {
        let command_res = Command::new("sh")
            .arg("-c")
            .arg("bluetoothctl show | grep 'Powered: yes'")
            .output()
            .expect("failed to execute process");
        let power_off = String::from_utf8_lossy(&command_res.stdout).is_empty();

        let options: Vec<_>;

        if power_off {
            options = vec!["Power: no"];
        } else {
            options = vec!["Power: yes"];
        }
        match rofi::Rofi::new(&options).prompt("Bluetooth").run() {
            Ok(choice) => match choice.as_str() {
                "Power: no" | "Power: yes" => toggle_power(power_off),
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
