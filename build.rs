use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use tinybmp::Bmp;

pub const BPP: u32 = 4;
pub const TILE_SIZE: u32 = 8;

fn main() {
    let bmp = include_bytes!("src/img/spritesheet.bmp");
    let image = Bmp::<Rgb888>::from_slice(bmp).expect("Failed to parse BMP file");
    let mut buffer = {
        let size = image.size().width * image.size().height;
        vec![0u8; size as usize]
    };

    let tilewise_width = image.size().width / TILE_SIZE;
    let tilewise_height = image.size().height / TILE_SIZE;

    /*
        These are 4bpp pixels, where each bit corresponds to an index in the palette. The
        red channel stores the index, while the green and blue channels are just used for
        visual distinction when editing the image.

        Each tile is made up by a block of 8x8 pixels in the spritesheet, but need to be
        contiguous in memory, so the (x, y) position of the tile in the spritesheet needs
        to be mapped to a linear index in the buffer. Ie:
         * (0, 0) -> 0
         * (1, 0) -> 1
         * ...
         * (8, 0) -> 64
         * (9, 0) -> 65
         * ...
         * (0, 1) -> 8
         * (1, 1) -> 9
         * ...
         * (8, 1) -> 72
         * (9, 1) -> 73
         * ...
    */
    for Pixel(position, color) in image.pixels() {
        let color = color.r();
        let x = position.x as u32;
        let y = position.y as u32;
        let tile_x = x / TILE_SIZE;
        let tile_y = y / TILE_SIZE;
        let tile_index = tile_x + tile_y * tilewise_width;
        let tile_offset = (x % TILE_SIZE) + (y % TILE_SIZE) * TILE_SIZE;
        let buffer_index = tile_index * TILE_SIZE * TILE_SIZE + tile_offset;
        buffer[buffer_index as usize] = color;
    }

    // Let's look at the buffer
    // println!("Uncompacted Buffer:");
    // for i in buffer.chunks_exact(8) {
    //     for j in i {
    //         print!("{}", j);
    //     }

    //     println!();
    // }

    /*
        Now that we have tiles stored contiguously in memory, we need to compact them
        into 32-bit words, where each word contains 8 pixels (4bpp). The GBA is little-endian,
        so we need to store the pixels in reverse order. Ie:
         * 0 1 2 3 4 5 6 7 -> 0x76543210
         * 8 9 A B C D E F -> 0xFEDCBA98
         * ...
    */
    let buffer = buffer.chunks_exact(8).fold(Vec::new(), |mut acc, chunk| {
        let mut word = 0u32;
        for (i, &pixel) in chunk.iter().enumerate() {
            word |= (pixel as u32) << (i * BPP as usize);
        }

        acc.push(word);
        acc
    });

    // Let's look at the buffer again
    // println!("Compacted Buffer:");
    // for i in buffer.iter() {
    //     // Print this 32-bit word as an 8-digit hex number
    //     println!("{:08x}", i);
    // }

    // Now, let's write the buffer to a file in OUT_DIR so we can import it in our binary
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("tiles.bin");

    // Transmute the buffer to a byte slice so we can write it to a file, we've already
    // accounded for the endianness when compacting the buffer

    let buffer = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr() as *const u8,
            buffer.len() * core::mem::size_of::<u32>(),
        )
    };

    std::fs::write(&path, buffer).expect("Failed to write tiles.bin");

    // Write this path to an environment variable so we can access it in our binary
    println!("cargo:rerun-if-changed=src/img/spritesheet.bmp");
    println!("cargo:rustc-env=TILES_BIN={}", path.display());
}
