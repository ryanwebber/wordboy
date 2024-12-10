#![no_std]
#![no_main]

use ab_os::{
    mmio::{BACKDROP, DISPCNT, KEYINPUT, OBJ_ATTRS, OBJ_PALETTE, OBJ_TILE4},
    video::{Color, DisplayControl, ObjAttr, Tile4, TileSize},
};

mod spritesheet;

#[no_mangle]
pub extern "C" fn main() -> ! {
    initialize_palette();

    OBJ_PALETTE.index(1).write(Color::RED);
    OBJ_PALETTE.index(2).write(Color::YELLOW);
    OBJ_PALETTE.index(3).write(Color::GREEN);
    OBJ_PALETTE.index(4).write(Color::BLUE);

    {
        let tiles = spritesheet::tile_16x16(22);
        for (i, tile) in tiles.iter().enumerate() {
            OBJ_TILE4.index(i + 1).write(*tile);
        }

        let obj = ObjAttr::new()
            .size(TileSize::SIZE_16X16)
            .tile(1)
            .palette(0)
            .x(100)
            .y(123);

        OBJ_ATTRS.index(0).write(obj);
    }

    {
        OBJ_TILE4.index(1 + 4).write(EXAMPLE_TILE);
        OBJ_TILE4.index(2 + 4).write(EXAMPLE_TILE);
        OBJ_TILE4.index(3 + 4).write(EXAMPLE_TILE);
        OBJ_TILE4.index(4 + 4).write(EXAMPLE_TILE);

        let obj = ObjAttr::new()
            .size(TileSize::SIZE_16X16)
            .tile(5)
            .palette(0)
            .x(15)
            .y(58);

        OBJ_ATTRS.index(1).write(obj);
    }

    DISPCNT.write(DisplayControl::ENABLE_OBJ | DisplayControl::LINEAR_OBJ_TILE_DATA);

    loop {
        let k = KEYINPUT.read();
        BACKDROP.write(if k.a() { Color::BLACK } else { Color::WHITE })
    }
}

fn initialize_palette() {
    for idx in OBJ_PALETTE.iter() {
        idx.write(Color::WHITE);
    }
}

/// A tile with an extra notch on the upper left.
#[rustfmt::skip]
const EXAMPLE_TILE: Tile4 = [
  // Each hex digit is one 4bpp index value.
  // Also, the image is left-right flipped from how it
  // looks in code because the GBA is little-endian!
  0x11111111,
  0x12222111,
  0x12222111,
  0x12222221,
  0x12222221,
  0x12222221,
  0x12222221,
  0x11111111,
];
