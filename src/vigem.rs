use crate::*;
use anyhow::Result;
use vigem_client::*;

pub struct VirtualGamepad {
    target: Xbox360Wired<Client>,
    state: XGamepad,
}

impl VirtualGamepad {
    pub fn new(g_type: GamepadType) -> Result<Self> {
        assert_eq!(g_type, GamepadType::DS4);

        // Connect to the ViGEmBus driver
        let client = Client::connect()?;

        // Create the virtual controller target
        let id = vigem_client::TargetId::XBOX360_WIRED;
        let mut target = Xbox360Wired::new(client, id);

        // Plugin the virtual controller
        target.plugin()?;

        // Wait for the virtual controller to be ready to accept updates
        target.wait_ready()?;

        // The input state of the virtual controller
        let state = XGamepad::default();

        Ok(Self { target, state })
    }
}

impl Backend for VirtualGamepad {
    fn key(&mut self, key: Key, pressed: bool) -> Result<()> {
        let bit = match key {
            Key::A => XButtons::A,
            Key::B => XButtons::B,
            Key::X => XButtons::X,
            Key::Y => XButtons::Y,
            Key::Up => XButtons::UP,
            Key::Down => XButtons::DOWN,
            Key::Left => XButtons::LEFT,
            Key::Right => XButtons::RIGHT,
            Key::Back => XButtons::BACK,
            Key::Start => XButtons::START,
            Key::Guide => XButtons::GUIDE,
            Key::L => XButtons::LB,
            Key::R => XButtons::RB,
            Key::L3 => XButtons::LTHUMB,
            Key::R3 => XButtons::RTHUMB,
        };
        if pressed {
            self.state.buttons.raw |= bit;
        } else {
            self.state.buttons.raw ^= bit;
        }
        Ok(())
    }

    fn axis(&mut self, axis: Axis, value: f64) -> Result<()> {
        match axis {
            Axis::ZL => self.state.left_trigger = (value * u8::MAX as f64) as u8,
            Axis::ZR => self.state.right_trigger = (value * u8::MAX as f64) as u8,
            Axis::Lx => self.state.thumb_lx = (value * i16::MAX as f64) as i16,
            Axis::Ly => self.state.thumb_ly = (value * i16::MAX as f64) as i16,
            Axis::Rx => self.state.thumb_rx = (value * i16::MAX as f64) as i16,
            Axis::Ry => self.state.thumb_ry = (value * i16::MAX as f64) as i16,
        }
        Ok(())
    }

    fn push(&mut self) -> Result<()> {
        Ok(self.target.update(&self.state)?)
    }
}
