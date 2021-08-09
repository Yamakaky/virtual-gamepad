#[cfg_attr(target_os = "linux", path = "uinput.rs")]
#[cfg_attr(windows, path = "vigem.rs")]
mod os;

use anyhow::Result;

pub enum GamepadType {
    Xbox360,
    DS4,
}

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

pub use os::*;
