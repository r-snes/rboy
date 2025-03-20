#![crate_name = "rboy"]
#![crate_type = "lib"]

pub use crate::gpu::{SCREEN_H, SCREEN_W};
pub use crate::keypad::KeypadKey;
pub use crate::sound::AudioPlayer;
pub use crate::cpu::CPU_FREQUENCY;

pub mod device;
pub mod plugins;

mod cpu;
mod gbmode;
mod gpu;
mod keypad;
mod mbc;
mod mmu;
mod printer;
mod register;
mod serial;
mod sound;
mod timer;

pub type StrResult<T> = Result<T, &'static str>;
