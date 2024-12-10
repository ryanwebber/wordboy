use voladdress::{Safe, VolAddress, VolBlock, VolSeries};

use crate::{
    input::KeyInput,
    video::{
        Color, DisplayControl, ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, Tile4, Tile8,
        OBJ_TILE_MEM_WORD_COUNT,
    },
};

pub const DISPCNT: VolAddress<DisplayControl, Safe, Safe> = unsafe { VolAddress::new(0x0400_0000) };
pub const KEYINPUT: VolAddress<KeyInput, Safe, ()> = unsafe { VolAddress::new(0x0400_0130) };

pub const BACKDROP: VolAddress<Color, Safe, Safe> = unsafe { VolAddress::new(0x0500_0000) };

pub const OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256> = unsafe { VolBlock::new(0x0500_0200) };
pub const OBJ_TILES: VolBlock<u32, Safe, Safe, OBJ_TILE_MEM_WORD_COUNT> =
    unsafe { VolBlock::new(0x0601_0000) };
pub const OBJ_TILE4: VolBlock<Tile4, Safe, Safe, 1024> = unsafe { VolBlock::new(0x0601_0000) };
pub const OBJ_TILE8: VolSeries<Tile8, Safe, Safe, 1023, 32> =
    unsafe { VolSeries::new(0x0601_0000) };

pub const OBJ_ATTRS_0: VolSeries<ObjAttr0, Safe, Safe, 128, 64> =
    unsafe { VolSeries::new(0x0700_0000) };
pub const OBJ_ATTRS_1: VolSeries<ObjAttr1, Safe, Safe, 128, 64> =
    unsafe { VolSeries::new(0x0700_0000 + 2) };
pub const OBJ_ATTRS_2: VolSeries<ObjAttr2, Safe, Safe, 128, 64> =
    unsafe { VolSeries::new(0x0700_0000 + 4) };

pub const OBJ_ATTRS: VolSeries<ObjAttr, Safe, Safe, 128, 8> =
    unsafe { VolSeries::new(0x0700_0000) };
