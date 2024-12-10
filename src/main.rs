#![no_std]
#![no_main]

use ab_os::{
    mmio::{BACKDROP, DISPCNT, KEYINPUT, OBJ_ATTRS, OBJ_PALETTE, OBJ_TILE4},
    video::{Color, DisplayControl, ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, Tile4, TileSize},
};

const TILES: &'static [u8] = include_bytes!(env!("TILES_BIN"));

fn tile(index: usize) -> Tile4 {
    let start = index * 32;
    let end = start + 8;
    let data = &TILES[start..end];

    // Cast slice to a u32 array
    let data = unsafe { core::mem::transmute::<&[u8], &[u32]>(data) };
    data.try_into().unwrap_or_else(|_| [1; 8])
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    initialize_palette();

    OBJ_PALETTE.index(1).write(Color::RED);
    OBJ_PALETTE.index(2).write(Color::YELLOW);
    OBJ_PALETTE.index(3).write(Color::GREEN);
    OBJ_PALETTE.index(4).write(Color::BLUE);

    {
        OBJ_TILE4.index(1).write(tile(0));
        OBJ_TILE4.index(2).write(tile(1));
        OBJ_TILE4.index(3).write(tile(2));
        OBJ_TILE4.index(4).write(tile(3));

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
            .tile(2)
            .palette(0)
            .x(15)
            .y(58);

        // OBJ_ATTRS.index(1).write(obj);

        OBJ_ATTRS.index(1).write(ObjAttr(
            ObjAttr0(0),       // square shape
            ObjAttr1(1 << 14), // size 1
            ObjAttr2(5),       // base tile 1
        ));
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
