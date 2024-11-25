#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u16);

impl Color {
    pub const RED: Self = Self::rgb(31, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 31, 0);

    #[inline]
    pub const fn rgb(r: u16, g: u16, b: u16) -> Self {
        Self(r | (g << 5) | (b << 10))
    }
}
