use std::time::Duration;

use virtual_gamepad::{Backend, Key};

fn main() -> anyhow::Result<()> {
    let mut dev = virtual_gamepad::new(virtual_gamepad::GamepadType::DS4)?;

    let mut x = 1.;
    loop {
        x = 1. - x;
        //dev.axis(virtual_gamepad::Axis::Ly, x)?;
        dev.key(Key::A, true)?;
        dev.push()?;
        std::thread::sleep(Duration::from_secs(1));
        dev.key(Key::A, false)?;
        dev.push()?;
        std::thread::sleep(Duration::from_secs(1));
    }
}
