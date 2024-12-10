use bitfrob::u16_with_value;

pub const BITS_PER_BYTE: usize = 8;
pub const PIXELS_PER_TILE: usize = 8 * 8;
pub const SIZE_OF_TILE4: usize = (PIXELS_PER_TILE * 4) / BITS_PER_BYTE;
pub const SIZE_OF_TILE8: usize = (PIXELS_PER_TILE * 8) / BITS_PER_BYTE;
pub const SIZE_OF_OBJ_TILE_MEM: usize = 32 * 1024;
pub const TILE4_WORD_COUNT: usize = SIZE_OF_TILE4 / core::mem::size_of::<u32>();
pub const TILE8_WORD_COUNT: usize = SIZE_OF_TILE8 / core::mem::size_of::<u32>();
pub const OBJ_TILE_MEM_WORD_COUNT: usize = SIZE_OF_OBJ_TILE_MEM / core::mem::size_of::<u32>();

pub type Tile4 = [u32; TILE4_WORD_COUNT];
pub type Tile8 = [u32; TILE8_WORD_COUNT];

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u16);

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 31);
    pub const GREEN: Self = Self::rgb(0, 31, 0);
    pub const CYAN: Self = Self::rgb(0, 31, 31);
    pub const RED: Self = Self::rgb(31, 0, 0);
    pub const MAGENTA: Self = Self::rgb(31, 0, 31);
    pub const YELLOW: Self = Self::rgb(31, 31, 0);
    pub const WHITE: Self = Self::rgb(31, 31, 31);

    #[inline]
    pub const fn rgb(r: u16, g: u16, b: u16) -> Self {
        Self(r | (g << 5) | (b << 10))
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DisplayControl: u16 {
        const FRAME_SELECT = 1 << 4;
        const HBLANK_INTERVAL_FREE = 1 << 5;
        const LINEAR_OBJ_TILE_DATA = 1 << 6;
        const FORCED_BLANK = 1 << 7;
        const ENABLE_BG0 = 1 << 8;
        const ENABLE_BG1 = 1 << 9;
        const ENABLE_BG2 = 1 << 10;
        const ENABLE_BG3 = 1 << 11;
        const ENABLE_OBJ = 1 << 12;
        const WINDOW_0_DISPLAY = 1 << 13;
        const WINDOW_1_DISPLAY = 1 << 14;
        const OBJ_WINDOW_DISPLAY = 1 << 15;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ObjAttr0(pub u16);
impl ObjAttr0 {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn shape(self, shape: u16) -> Self {
        Self(u16_with_value(14, 15, self.0, shape))
    }

    #[inline]
    pub const fn y(self, y: i16) -> Self {
        Self(u16_with_value(0, 7, self.0, y as u16))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ObjAttr1(pub u16);
impl ObjAttr1 {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn size(self, size: u16) -> Self {
        Self(u16_with_value(14, 15, self.0, size))
    }

    #[inline]
    pub const fn x(self, x: i16) -> Self {
        Self(u16_with_value(0, 9, self.0, x as u16))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ObjAttr2(pub u16);
impl ObjAttr2 {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn tile(self, tile: u16) -> Self {
        Self(u16_with_value(0, 9, self.0, tile))
    }

    #[inline]
    pub const fn palette(self, bank: u16) -> Self {
        Self(u16_with_value(12, 15, self.0, bank))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjAttr(pub ObjAttr0, pub ObjAttr1, pub ObjAttr2);

impl ObjAttr {
    #[inline]
    pub const fn new() -> Self {
        Self(ObjAttr0::new(), ObjAttr1::new(), ObjAttr2::new())
    }

    #[inline]
    pub const fn size(self, size: TileSize) -> Self {
        Self(
            self.0.shape(size.0 as u16),
            self.1.size(size.1 as u16),
            self.2,
        )
    }

    #[inline]
    pub const fn tile(self, tile: u16) -> Self {
        Self(self.0, self.1, self.2.tile(tile))
    }

    pub const fn palette(self, bank: u16) -> Self {
        Self(self.0, self.1, self.2.palette(bank))
    }

    #[inline]
    pub const fn x(self, x: i16) -> Self {
        Self(self.0, self.1.x(x), self.2)
    }

    #[inline]
    pub const fn y(self, y: i16) -> Self {
        Self(self.0.y(y), self.1, self.2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TileSize(u8, u8);

impl TileSize {
    pub const SIZE_8X8: Self = Self(0b00, 0b00);
    pub const SIZE_16X16: Self = Self(0b00, 0b01);
    pub const SIZE_32X32: Self = Self(0b00, 0b10);
    pub const SIZE_64X64: Self = Self(0b00, 0b11);
    pub const SIZE_16X8: Self = Self(0b01, 0b00);
    pub const SIZE_32X8: Self = Self(0b01, 0b01);
    pub const SIZE_32X16: Self = Self(0b01, 0b10);
    pub const SIZE_64X32: Self = Self(0b01, 0b11);
    pub const SIZE_8X16: Self = Self(0b10, 0b00);
    pub const SIZE_8X32: Self = Self(0b10, 0b01);
    pub const SIZE_16X32: Self = Self(0b10, 0b10);
    pub const SIZE_32X64: Self = Self(0b10, 0b11);
}
