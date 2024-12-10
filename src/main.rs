#![no_std]
#![no_main]

use ab_os::{
    mmio::{BACKDROP, DISPCNT, KEYINPUT, OBJ_ATTRS, OBJ_PALETTE, OBJ_TILE4},
    video::{Color, DisplayControl, ObjAttr, TileSize},
};

mod spritesheet;

fn draw_tile(offset: usize, index: usize, palette: u16, x: i16, y: i16) {
    let tiles = spritesheet::tile_16x16(index);
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

    const SCREEN_WIDTH: i16 = 240;
    const SCREEN_HEIGHT: i16 = 160;
    const TILE_PADDING: i16 = 4;
    const TILE_COL_COUNT: i16 = 5;
    const TILE_ROW_COUNT: i16 = 6;
    const TILE_WIDTH: i16 = 16;
    const ROW_WIDTH: i16 = (TILE_COL_COUNT * TILE_WIDTH) + (TILE_PADDING * TILE_COL_COUNT - 1);
    const ROW_OFFSET: i16 = (SCREEN_WIDTH - ROW_WIDTH) / 2;

    let mut o = 0;
    // Draw 5-tiles across, 6-tiles down
    for i in 0..TILE_ROW_COUNT {
        for j in 0..TILE_COL_COUNT {
            draw_tile(
                o,
                o % 26,
                3,
                ROW_OFFSET + (j as i16) * (TILE_WIDTH + TILE_PADDING),
                TILE_PADDING + (i as i16) * (TILE_WIDTH + TILE_PADDING),
            );

            o += 1;
        }
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
    OBJ_PALETTE.index(16 * 1 + 2).write(Color::rgb(9, 20, 16));
    OBJ_PALETTE.index(16 * 1 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 1 + 4).write(Color::rgb(7, 15, 10));

    // Palette Bank 2 : Yellow letters
    OBJ_PALETTE.index(16 * 2 + 1).write(Color::WHITE);
    OBJ_PALETTE.index(16 * 2 + 2).write(Color::rgb(25, 23, 12));
    OBJ_PALETTE.index(16 * 2 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 2 + 4).write(Color::rgb(17, 15, 2));

    // Palette Bank 3 : Gray letters
    OBJ_PALETTE.index(16 * 3 + 1).write(Color::WHITE);
    OBJ_PALETTE.index(16 * 3 + 2).write(Color::rgb(13, 15, 15));
    OBJ_PALETTE.index(16 * 3 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 3 + 4).write(Color::rgb(8, 10, 10));
}

fn initialize_display() {
    DISPCNT.write(DisplayControl::ENABLE_OBJ | DisplayControl::LINEAR_OBJ_TILE_DATA);
}
