#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyInput(pub u16);

impl KeyInput {
    #[inline]
    pub const fn a(self) -> bool {
        (self.0 & (1 << 0)) == 0
    }

    #[inline]
    pub const fn a_once(self, old: Self) -> bool {
        self.a() && !old.a()
    }

    #[inline]
    pub const fn b(self) -> bool {
        (self.0 & (1 << 1)) == 0
    }

    #[inline]
    pub const fn b_once(self, old: Self) -> bool {
        self.b() && !old.b()
    }

    #[inline]
    pub const fn select(self) -> bool {
        (self.0 & (1 << 2)) == 0
    }

    #[inline]
    pub const fn select_once(self, old: Self) -> bool {
        self.select() && !old.select()
    }

    #[inline]
    pub const fn start(self) -> bool {
        (self.0 & (1 << 3)) == 0
    }

    #[inline]
    pub const fn start_once(self, old: Self) -> bool {
        self.start() && !old.start()
    }

    #[inline]
    pub const fn right(self) -> bool {
        (self.0 & (1 << 4)) == 0
    }

    #[inline]
    pub const fn right_once(self, old: Self) -> bool {
        self.right() && !old.right()
    }

    #[inline]
    pub const fn left(self) -> bool {
        (self.0 & (1 << 5)) == 0
    }

    #[inline]
    pub const fn left_once(self, old: Self) -> bool {
        self.left() && !old.left()
    }

    #[inline]
    pub const fn up(self) -> bool {
        (self.0 & (1 << 6)) == 0
    }

    #[inline]
    pub const fn up_once(self, old: Self) -> bool {
        self.up() && !old.up()
    }

    #[inline]
    pub const fn down(self) -> bool {
        (self.0 & (1 << 7)) == 0
    }

    #[inline]
    pub const fn down_once(self, old: Self) -> bool {
        self.down() && !old.down()
    }

    #[inline]
    pub const fn r(self) -> bool {
        (self.0 & (1 << 8)) == 0
    }

    #[inline]
    pub const fn r_once(self, old: Self) -> bool {
        self.r() && !old.r()
    }

    #[inline]
    pub const fn l(self) -> bool {
        (self.0 & (1 << 9)) == 0
    }

    #[inline]
    pub const fn l_once(self, old: Self) -> bool {
        self.l() && !old.l()
    }
}
