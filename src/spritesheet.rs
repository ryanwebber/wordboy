use ab_os::video::Tile4;

const SPRITESHEET_WIDTH: usize = 8;
const TILES: &'static [u8] = include_bytes!(env!("TILES_BIN"));

pub fn tile_16x16(index: usize) -> [Tile4; 4] {
    /*
       The 16x16 tiles are made up of 4 8x8 tiles, where
       the top left and top right are on one row, and the bottom
       left and bottom right are on the next row, offset by
       the number of 8x8 tiles in a row.

       ie:
        * 0 => [0, 1, 16, 17]
        * 1 => [2, 3, 18, 19]
        * 8 => [32, 33, 48, 49]
        * 9 => [34, 35, 50, 51]
    */
    let row = index / SPRITESHEET_WIDTH;
    let col = index % SPRITESHEET_WIDTH;
    let top_left = 2 * (row * SPRITESHEET_WIDTH * 2 + col);
    let top_right = top_left + 1;
    let bottom_left = top_left + SPRITESHEET_WIDTH * 2;
    let bottom_right = bottom_left + 1;

    [
        tile_8x8(top_left),
        tile_8x8(top_right),
        tile_8x8(bottom_left),
        tile_8x8(bottom_right),
    ]
}

pub fn tile_8x8(index: usize) -> Tile4 {
    let start = index * 32;
    let end = start + 8;
    let data = &TILES[start..end];

    // Cast slice to a u32 array
    let data = unsafe { core::mem::transmute::<&[u8], &[u32]>(data) };
    data.try_into().unwrap_or_else(|_| [1; 8])
}
