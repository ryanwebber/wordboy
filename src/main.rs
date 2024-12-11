#![no_std]
#![no_main]

use ab_os::{
    mmio::{BACKDROP, DISPCNT, KEYINPUT, OBJ_ATTRS, OBJ_PALETTE, OBJ_TILE4},
    video::{Color, DisplayControl, ObjAttr, Tile4, TileSize},
};
use utils::WordBuffer;

mod dictionary;
mod game;
mod utils;

#[no_mangle]
pub extern "C" fn main() -> ! {
    initialize_display();
    initialize_palette();
    intiialize_sprites();

    let state = game::State::new(WordBuffer::from_str("HELLO"));
    state.render();

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

    // Palette Bank 4 : Black letters
    OBJ_PALETTE.index(16 * 4 + 1).write(Color::WHITE);
    OBJ_PALETTE.index(16 * 4 + 2).write(Color::BLACK);
    OBJ_PALETTE.index(16 * 4 + 3).write(Color::rgb(6, 6, 6));
    OBJ_PALETTE.index(16 * 4 + 4).write(Color::rgb(18, 18, 18));
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
