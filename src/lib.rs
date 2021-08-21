#[cfg_attr(target_os = "linux", path = "uinput.rs")]
#[cfg_attr(windows, path = "vigem.rs")]
mod os;

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadType {
    Xbox360,
    DS4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    A,
    B,
    X,
    Y,
    Up,
    Down,
    Left,
    Right,
    Back,
    Start,
    Guide,
    L,
    R,
    L3,
    R3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    ZL,
    ZR,
    Lx,
    Ly,
    Rx,
    Ry,
}

pub trait Backend {
    fn key(&mut self, key: Key, pressed: bool) -> Result<()>;
    fn axis(&mut self, axis: Axis, value: f64) -> Result<()>;
    fn push(&mut self) -> Result<()>;
}

struct NullBackend;
impl Backend for NullBackend {
    fn key(&mut self, _key: Key, _pressed: bool) -> Result<()> {
        bail!("unsupported OS")
    }

    fn axis(&mut self, _axis: Axis, _value: f64) -> Result<()> {
        bail!("unsupported OS")
    }

    fn push(&mut self) -> Result<()> {
        bail!("unsupported OS")
    }
}

#[allow(unused_variables)]
pub fn new(g_type: GamepadType) -> Result<impl Backend> {
    #[cfg(windows)]
    {
        os::VirtualGamepad::new(g_type)
    }

    #[cfg(target_os = "linux")]
    {
        os::VirtualGamepad::new(g_type)
    }

    #[cfg(not(any(windows, target_os = "linux")))]
    {
        bail!("not supported on this platform");
        #[allow(unreachable_code)]
        Ok(NullBackend)
    }
}
