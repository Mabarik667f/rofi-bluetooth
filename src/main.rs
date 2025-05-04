use std::env;

use ui::rofi::{BluetoothMenuResult, show_main_menu};

pub mod bluetooth;
pub mod ui;

fn main() {
    let args: Vec<String> = env::args().collect();
    let theme = if args.len() >= 2 { &args[1] } else { "Monokai" };

    loop {
        match show_main_menu(&theme) {
            BluetoothMenuResult::Continue => {
                continue;
            }
            BluetoothMenuResult::ExitWithError(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
