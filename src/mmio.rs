use voladdress::{Safe, VolAddress};

use crate::{input::KeyInput, video::Color};

pub const BACKDROP: VolAddress<Color, Safe, Safe> = unsafe { VolAddress::new(0x0500_0000) };
pub const DISPCNT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x0400_0000) };
pub const KEYINPUT: VolAddress<KeyInput, Safe, ()> = unsafe { VolAddress::new(0x400_0130) };
