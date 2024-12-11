#![no_std]
#![no_main]

use ab_os::{
    mmio::{BACKDROP, DISPCNT, KEYINPUT, OBJ_ATTRS, OBJ_PALETTE, OBJ_TILE4},
    video::{Color, DisplayControl, ObjAttr, Tile4, TileSize},
};

fn draw_tile(offset: usize, index: u16, palette: u16, x: i16, y: i16) {
    let obj = ObjAttr::new()
        .size(TileSize::SIZE_16X16)
        .tile(index * 4 + 1)
        .palette(palette)
        .x(x)
        .y(y);

    OBJ_ATTRS.index(offset).write(obj);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    initialize_display();
    initialize_palette();
    intiialize_sprites();

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
                o as u16,
                3,
                ROW_OFFSET + (j as i16) * (TILE_WIDTH + TILE_PADDING),
                TILE_PADDING * 2 + (i as i16) * (TILE_WIDTH + TILE_PADDING),
            );

            o += 1;
        }
    }

    for i in 0..TILE_COL_COUNT {
        draw_tile(
            o,
            27,
            3,
            ROW_OFFSET + (i as i16) * (TILE_WIDTH + TILE_PADDING),
            SCREEN_HEIGHT - TILE_WIDTH - TILE_PADDING * 2,
        );

        o += 1;
    }

    loop {
        let k = KEYINPUT.read();
        BACKDROP.write(if k.a() { Color::WHITE } else { Color::BLACK })
    }
}

fn initialize_display() {
    DISPCNT.write(DisplayControl::ENABLE_OBJ | DisplayControl::LINEAR_OBJ_TILE_DATA);
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

fn intiialize_sprites() {
    const SPRITES: &'static [u8] = include_bytes!(env!("SPRITES_BIN"));
    const SPRITESHEET_WIDTH: usize = 8;
    const SPRITESHEET_HEIGHT: usize = 8;

    /*
       Our 8x8 tiles need to be mapped linearly in memory to create
       16x16 tiles. I tried using 2D sprite mapping here, but it created
       some weird artifacts despite the debugger in mGBA showing the
       correct memory layout.
    */

    fn create_tile(index: usize) -> Tile4 {
        let start = index * 32;
        let end = start + 8;
        let data = &SPRITES[start..end];

        // Cast slice to a u32 array
        let data = unsafe { core::mem::transmute::<&[u8], &[u32]>(data) };
        data.try_into().unwrap_or_else(|_| [1; 8])
    }

    let mut tile_index = 0;
    for row in 0..SPRITESHEET_HEIGHT {
        for col in 0..SPRITESHEET_WIDTH {
            let top_left = 2 * (row * SPRITESHEET_WIDTH * 2 + col);
            let top_right = top_left + 1;
            let bottom_left = top_left + SPRITESHEET_WIDTH * 2;
            let bottom_right = bottom_left + 1;

            let tile = [
                create_tile(top_left),
                create_tile(top_right),
                create_tile(bottom_left),
                create_tile(bottom_right),
            ];

            for (i, tile) in tile.iter().enumerate() {
                OBJ_TILE4.index(tile_index + i + 1).write(*tile);
            }

            tile_index += 4;
        }
    }
}
