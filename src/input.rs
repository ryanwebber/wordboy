#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyInput(pub u16);

impl KeyInput {
    #[inline]
    pub const fn a(self) -> bool {
        (self.0 & (1 << 0)) == 0
    }

    #[inline]
    pub const fn b(self) -> bool {
        (self.0 & (1 << 1)) == 0
    }

    #[inline]
    pub const fn select(self) -> bool {
        (self.0 & (1 << 2)) == 0
    }

    #[inline]
    pub const fn start(self) -> bool {
        (self.0 & (1 << 3)) == 0
    }

    #[inline]
    pub const fn right(self) -> bool {
        (self.0 & (1 << 4)) == 0
    }

    #[inline]
    pub const fn left(self) -> bool {
        (self.0 & (1 << 5)) == 0
    }

    #[inline]
    pub const fn up(self) -> bool {
        (self.0 & (1 << 6)) == 0
    }

    #[inline]
    pub const fn down(self) -> bool {
        (self.0 & (1 << 7)) == 0
    }

    #[inline]
    pub const fn r(self) -> bool {
        (self.0 & (1 << 8)) == 0
    }

    #[inline]
    pub const fn l(self) -> bool {
        (self.0 & (1 << 9)) == 0
    }
}
