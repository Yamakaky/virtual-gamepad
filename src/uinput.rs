use anyhow::Result;
use evdev::uinput::*;
use evdev::*;

use crate::Backend;

pub struct VirtualGamepad {
    dev: VirtualDevice,
    queue: Vec<InputEvent>,
}

impl VirtualGamepad {
    pub fn new(name: &str) -> Result<Self> {
        let mut dev = VirtualDeviceBuilder::new()?
            .name(name)
            .input_id(InputId::new(BusType::BUS_USB, 0x054c, 0x09cc, 2));
        for axis in AXES {
            dev = dev.with_absolute_axes(*axis, i16::MIN as i32, i16::MAX as i32)?;
        }
        let dev = dev.with_keys(&Self::keys())?.build()?;
        Ok(VirtualGamepad {
            dev,
            queue: Vec::new(),
        })
    }

    pub fn dev(&mut self) -> &mut VirtualDevice {
        &mut self.dev
    }

    fn keys() -> AttributeSet<evdev::Key> {
        let mut set = AttributeSet::new();
        for key in KEYS {
            set.insert(*key);
        }
        set
    }
}

impl Backend for VirtualGamepad {
    fn key(&mut self, key: crate::Key, pressed: bool) -> Result<()> {
        let code = match key {
            crate::Key::A => evdev::Key::BTN_SOUTH,
            crate::Key::B => evdev::Key::BTN_EAST,
            crate::Key::X => evdev::Key::BTN_WEST,
            crate::Key::Y => evdev::Key::BTN_EAST,
            crate::Key::Up => evdev::Key::BTN_DPAD_UP,
            crate::Key::Down => evdev::Key::BTN_DPAD_DOWN,
            crate::Key::Left => evdev::Key::BTN_DPAD_LEFT,
            crate::Key::Right => evdev::Key::BTN_DPAD_RIGHT,
            crate::Key::Back => evdev::Key::BTN_BACK,
            crate::Key::Start => evdev::Key::BTN_START,
            crate::Key::Guide => evdev::Key::BTN_MODE,
            crate::Key::L => evdev::Key::BTN_TL,
            crate::Key::R => evdev::Key::BTN_TR,
            crate::Key::L3 => evdev::Key::BTN_THUMBL,
            crate::Key::R3 => evdev::Key::BTN_THUMBR,
        };
        self.queue
            .push(InputEvent::new(EventType::KEY, code.code(), pressed as i32));
        Ok(())
    }

    fn axis(&mut self, axis: crate::Axis, value: f64) -> Result<()> {
        let code = match axis {
            crate::Axis::Lx => AbsoluteAxisType::ABS_X,
            crate::Axis::Ly => AbsoluteAxisType::ABS_Y,
            crate::Axis::ZL => AbsoluteAxisType::ABS_Z,
            crate::Axis::Rx => AbsoluteAxisType::ABS_RX,
            crate::Axis::Ry => AbsoluteAxisType::ABS_RY,
            crate::Axis::ZR => AbsoluteAxisType::ABS_RZ,
        };
        self.queue.push(InputEvent::new(
            EventType::ABSOLUTE,
            code.0,
            (value * i16::MAX as f64) as i32,
        ));
        Ok(())
    }

    fn push(&mut self) -> Result<()> {
        self.dev.emit(&self.queue)?;
        self.queue.clear();
        Ok(())
    }
}

const KEYS: &[Key] = &[
    evdev::Key::BTN_SOUTH,
    evdev::Key::BTN_NORTH,
    evdev::Key::BTN_WEST,
    evdev::Key::BTN_EAST,
    evdev::Key::BTN_SELECT,
    evdev::Key::BTN_START,
    evdev::Key::BTN_MODE,
    evdev::Key::BTN_TL,
    evdev::Key::BTN_TR,
    evdev::Key::BTN_TL2,
    evdev::Key::BTN_TR2,
    evdev::Key::BTN_THUMBL,
    evdev::Key::BTN_THUMBR,
];

const AXES: &[AbsoluteAxisType] = &[
    AbsoluteAxisType::ABS_X,
    AbsoluteAxisType::ABS_Y,
    AbsoluteAxisType::ABS_Z,
    AbsoluteAxisType::ABS_RX,
    AbsoluteAxisType::ABS_RY,
    AbsoluteAxisType::ABS_RZ,
    AbsoluteAxisType::ABS_HAT0X,
    AbsoluteAxisType::ABS_HAT0Y,
];
