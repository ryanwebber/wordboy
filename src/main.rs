#![no_std]
#![no_main]

use ab_os::{
    mmio::{BACKDROP, DISPCNT, KEYINPUT, OBJ_ATTRS, OBJ_PALETTE, OBJ_TILE4},
    video::{Color, DisplayControl, ObjAttr, TileSize},
};

mod spritesheet;

fn draw_tile(offset: usize, letter: char, palette: u16, x: i16, y: i16) {
    let tiles = spritesheet::tile_16x16({
        // 'A' => 0, 'B' => 1, 'C' => 2, 'D' => 3, ...
        (letter as u8 - 'A' as u8) as usize
    });

    for (i, tile) in tiles.iter().enumerate() {
        OBJ_TILE4.index(i + 1 + offset * 4).write(*tile);
    }

    let obj = ObjAttr::new()
        .size(TileSize::SIZE_16X16)
        .tile(1 + (offset as u16) * 4)
        .palette(palette)
        .x(x)
        .y(y);

    OBJ_ATTRS.index(offset).write(obj);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    initialize_palette();
    initialize_display();

    for (i, c) in "HELLO".chars().enumerate() {
        draw_tile(i, c, ((i % 4) + 1) as u16, (i * 20 + 16) as i16, 16);
    }

    for (i, c) in "WORLD".chars().enumerate() {
        draw_tile(
            i + 5,
            c,
            (((i + 3) % 4) + 1) as u16,
            (i * 20 + 16) as i16,
            36,
        );
    }

    loop {
        let k = KEYINPUT.read();
        BACKDROP.write(if k.a() { Color::WHITE } else { Color::BLACK })
    }
}

fn initialize_palette() {
    for idx in OBJ_PALETTE.iter() {
        idx.write(Color::WHITE);
    }

    // Palette Bank 1 : Green letters
    OBJ_PALETTE.index(16 * 1 + 1).write(Color::WHITE);
    OBJ_PALETTE.index(16 * 1 + 2).write(Color::rgb(3, 20, 8));
    OBJ_PALETTE.index(16 * 1 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 1 + 4).write(Color::rgb(2, 15, 4));

    // Palette Bank 2 : Yellow letters
    OBJ_PALETTE.index(16 * 2 + 1).write(Color::WHITE);
    OBJ_PALETTE.index(16 * 2 + 2).write(Color::rgb(23, 18, 3));
    OBJ_PALETTE.index(16 * 2 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 2 + 4).write(Color::rgb(17, 13, 2));

    // Palette Bank 3 : Red letters
    OBJ_PALETTE.index(16 * 3 + 1).write(Color::WHITE);
    OBJ_PALETTE.index(16 * 3 + 2).write(Color::rgb(20, 5, 8));
    OBJ_PALETTE.index(16 * 3 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 3 + 4).write(Color::rgb(13, 2, 2));

    // Palette Bank 4 : Gray letters
    OBJ_PALETTE.index(16 * 4 + 1).write(Color::WHITE);
    OBJ_PALETTE.index(16 * 4 + 2).write(Color::rgb(13, 15, 15));
    OBJ_PALETTE.index(16 * 4 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 4 + 4).write(Color::rgb(8, 10, 10));
}

fn initialize_display() {
    DISPCNT.write(DisplayControl::ENABLE_OBJ | DisplayControl::LINEAR_OBJ_TILE_DATA);
}
