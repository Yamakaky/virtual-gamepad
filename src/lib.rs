#[cfg_attr(target_os = "linux", path = "uinput.rs")]
#[cfg_attr(windows, path = "vigem.rs")]
mod os;

#[cfg(target_os = "macos")]
mod os {}

use os::*;

use anyhow::Result;

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

#[allow(unused_variables)]
pub fn new(g_type: GamepadType) -> Result<impl Backend> {
    #[cfg(windows)]
    {
        VirtualGamepad::new(g_type)
    }

    #[cfg(target_os = "linux")]
    {
        VirtualGamepad::new(g_type)
    }

    #[cfg(not(any(windows, target_os = "linux")))]
    {
        anyhow::bail!("not supported on this platform")
    }
}
