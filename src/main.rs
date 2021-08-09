use virtual_gamepad::{Backend, Key};

fn main() -> anyhow::Result<()> {
    let mut dev =
        virtual_gamepad::VirtualGamepad::new("Sony Interactive Entertainment Wireless Controller")?;

    let mut x = 1.;
    loop {
        x = 1. - x;
        //dev.axis(virtual_gamepad::Axis::Ly, x)?;
        dev.key(Key::A, true)?;
        dev.push()?;
        std::thread::sleep_ms(1000);
        dev.key(Key::A, false)?;
        dev.push()?;
        std::thread::sleep_ms(1000);
    }
}
