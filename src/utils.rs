use core::mem::MaybeUninit;

pub struct ArrayVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: core::array::from_fn(|_| unsafe { MaybeUninit::uninit().assume_init() }),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn nth(&self, n: usize) -> Option<&T> {
        if n < self.len {
            Some(unsafe { self.data[n].assume_init_ref() })
        } else {
            None
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            Some(unsafe { self.data[self.len - 1].assume_init_mut() })
        } else {
            None
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len < N {
            self.data[self.len] = MaybeUninit::new(value);
            self.len += 1;
        } else {
            panic!("array is full")
        }
    }

    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len < N {
            self.data[self.len] = MaybeUninit::new(value);
            self.len += 1;
            Ok(())
        } else {
            Err(value)
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.data
            .iter()
            .take(self.len)
            .map(|x| unsafe { x.assume_init_ref() })
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AsciiChar(pub u8);

impl AsciiChar {
    pub const NULL: Self = Self(0x00);

    pub const fn from_u8(byte: u8) -> Self {
        if byte >= b'A' && byte <= b'Z' {
            Self(byte)
        } else if byte >= b'a' && byte <= b'z' {
            Self(b'A' + (byte - b'a'))
        } else {
            Self::NULL
        }
    }

    pub fn letter_index(self) -> u16 {
        if self.0 >= b'A' && self.0 <= b'Z' {
            return (self.0 - b'A') as u16;
        } else {
            26
        }
    }

    pub fn tile_index(self) -> u16 {
        self.letter_index() * 4 + 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WordBuffer(pub [AsciiChar; 5]);

impl WordBuffer {
    pub const EMPTY: Self = Self([AsciiChar::NULL; 5]);

    pub const fn from_u8s(value: [u8; 5]) -> Self {
        let mut buffer = [AsciiChar::NULL; 5];
        buffer[0] = AsciiChar::from_u8(value[0]);
        buffer[1] = AsciiChar::from_u8(value[1]);
        buffer[2] = AsciiChar::from_u8(value[2]);
        buffer[3] = AsciiChar::from_u8(value[3]);
        buffer[4] = AsciiChar::from_u8(value[4]);
        Self(buffer)
    }

    pub fn as_slice(&self) -> &[AsciiChar] {
        &self.0
    }

    pub fn is_full(&self) -> bool {
        self.0.iter().all(|&c| c != AsciiChar::NULL)
    }

    pub fn push(&mut self, letter: AsciiChar) {
        if self.is_full() {
            return;
        }

        for c in self.0.iter_mut() {
            if *c == AsciiChar::NULL {
                *c = letter;
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<AsciiChar> {
        for c in self.0.iter_mut().rev() {
            if *c != AsciiChar::NULL {
                let letter = *c;
                *c = AsciiChar::NULL;
                return Some(letter);
            }
        }

        None
    }

    pub fn clear(&mut self) {
        for c in self.0.iter_mut() {
            *c = AsciiChar::NULL;
        }
    }
}
