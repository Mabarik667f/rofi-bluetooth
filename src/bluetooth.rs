pub mod controller;
pub mod device;

use std::process::{Command, Output};

#[derive(Debug)]
pub struct ServiceAction<'a> {
    arg: &'a str,
    err_msg: &'a str,
}

pub trait BluetoothCtlRunner {
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
