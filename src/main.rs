use ui::rofi::{BluetoothMenuResult, show_main_menu};

pub mod bluetooth;
pub mod ui;

fn main() {
    loop {
        match show_main_menu() {
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
