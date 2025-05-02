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
    let flag = if power_off { "on" } else { "off" };
    ServiceAction {
        arg: format!("bluetoothctl power {flag}").as_str(),
        err_msg: "failed to toggle power",
    }
    .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_on_power() {
        let output = toggle_power(true);
        println!("{:?}", output);
    }

    #[test]
    fn toggle_of_power() {
        let output = toggle_power(false);
        println!("{:?}", output);
    }
}
