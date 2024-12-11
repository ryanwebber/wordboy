use ab_os::{
    mmio::OBJ_ATTRS,
    video::{ObjAttr, TileSize},
};

use crate::{
    dictionary,
    utils::{ArrayVec, AsciiChar, WordBuffer},
};

const SCREEN_WIDTH: i16 = 240;
const SCREEN_HEIGHT: i16 = 160;
const TILE_PADDING: i16 = 4;
const TILE_COL_COUNT: i16 = 5;
const TILE_ROW_COUNT: usize = 6;
const TILE_WIDTH: i16 = 16;
const ROW_WIDTH: i16 = (TILE_COL_COUNT * TILE_WIDTH) + (TILE_PADDING * (TILE_COL_COUNT - 1));
const ROW_OFFSET: i16 = (SCREEN_WIDTH - ROW_WIDTH) / 2;

const GREEN_PALETTE: u16 = 1;
const YELLOW_PALETTE: u16 = 2;
const GREY_PALETTE: u16 = 3;
const BLACK_PALETTE: u16 = 4;

const NULL_TILE: u16 = 47;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GuessState {
    Grey = 0,
    Yellow = 1,
    Green = 2,
    Black = 3,
}

impl GuessState {
    fn maybe_upgrade(&mut self, new_state: Self) {
        if (*self as u8) < (new_state as u8) {
            *self = new_state;
        }
    }

    fn palette_index(&self) -> u16 {
        match self {
            Self::Grey => GREY_PALETTE,
            Self::Yellow => YELLOW_PALETTE,
            Self::Green => GREEN_PALETTE,
            Self::Black => BLACK_PALETTE,
        }
    }
}

enum Input {
    Char,
    CursorLeft,
    CursorRight,
    Key(AsciiChar),
    Delete,
    Submit,
}

pub struct State {
    word: WordBuffer,
    guesses: ArrayVec<WordBuffer, TILE_ROW_COUNT>,
    letter_states: [GuessState; 27],
    cursor: u8,
}

impl State {
    pub fn new(word: WordBuffer) -> Self {
        let mut this = Self {
            word,
            guesses: {
                let mut guesses = ArrayVec::new();
                guesses.push(WordBuffer::EMPTY);
                guesses
            },
            letter_states: [GuessState::Grey; 27],
            cursor: 0,
        };

        this.input(Input::Key(AsciiChar(b'H')));
        this.input(Input::Key(AsciiChar(b'G')));
        this.input(Input::Key(AsciiChar(b'A')));
        this.input(Input::Delete);
        this.input(Input::Key(AsciiChar(b'B')));
        this.input(Input::Key(AsciiChar(b'V')));
        this.input(Input::Key(AsciiChar(b'E')));
        this.input(Input::Submit);
        this.input(Input::Key(AsciiChar(b'Y')));
        this.input(Input::Key(AsciiChar(b'Q')));
        this.input(Input::CursorRight);
        this.input(Input::CursorRight);
        this.input(Input::CursorLeft);
        this.input(Input::Char);

        this
    }

    fn input(&mut self, input: Input) {
        match input {
            Input::Char => {
                let char = AsciiChar(b'A' + self.cursor);
                self.input(Input::Key(char));
            }
            Input::CursorLeft => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            Input::CursorRight => {
                if self.cursor < 25 {
                    self.cursor += 1;
                }
            }
            Input::Delete => {
                let current_guess = self.guesses.last_mut().unwrap();
                current_guess.pop();
            }
            Input::Key(letter) => {
                let current_guess = self.guesses.last_mut().unwrap();
                if current_guess.is_full() {
                    return;
                }

                current_guess.push(letter);
            }
            Input::Submit => {
                let current_guess = self.guesses.last_mut().unwrap();
                if !current_guess.is_full() {
                    return;
                }

                // Check if the guess is valid or not
                if !dictionary::valid_guess(&current_guess) {
                    current_guess.clear();
                    return;
                }

                // Update the keyboard colors
                for (i, c) in current_guess.as_slice().iter().enumerate() {
                    if self.word.as_slice()[i] == *c {
                        self.letter_states[c.letter_index() as usize]
                            .maybe_upgrade(GuessState::Green);
                    } else if self.word.as_slice().contains(c) {
                        self.letter_states[c.letter_index() as usize]
                            .maybe_upgrade(GuessState::Yellow);
                    } else {
                        self.letter_states[c.letter_index() as usize] = GuessState::Black;
                    }
                }

                // Add a new guess if the current one is full
                self.guesses.push(WordBuffer::EMPTY);
            }
        }
    }

    pub fn render(&self) {
        fn draw_guessed_tile(char: AsciiChar, row: usize, col: usize, palette: u16) {
            let x = ROW_OFFSET + (col as i16) * (TILE_WIDTH + TILE_PADDING);
            let y = TILE_PADDING + (row as i16) * (TILE_WIDTH + TILE_PADDING);

            let obj = ObjAttr::new()
                .size(TileSize::SIZE_16X16)
                .tile(char.tile_index())
                .palette(palette)
                .x(x)
                .y(y);

            // This can't be overlapped!
            let offset = row * 5 + col;

            OBJ_ATTRS.index(offset).write(obj);
        }

        for (row, word) in self.guesses.iter().enumerate() {
            for (col, char) in word.as_slice().iter().enumerate() {
                let palette_index = if *char == AsciiChar::NULL || row == self.guesses.len() - 1 {
                    BLACK_PALETTE
                } else if self.word.as_slice()[col] == *char {
                    GREEN_PALETTE
                } else if self.word.as_slice().contains(char) {
                    YELLOW_PALETTE
                } else {
                    GREY_PALETTE
                };

                draw_guessed_tile(*char, row, col, palette_index);
            }
        }

        let unused_guess_count = self.guesses.capacity() - self.guesses.len();
        for i in 0..unused_guess_count {
            for j in 0..5 {
                draw_guessed_tile(AsciiChar::NULL, i + self.guesses.len(), j, BLACK_PALETTE);
            }
        }

        // Render the keyboard. The cursor is always in the middle,
        // so we need to adjust the position of the tiles so whatever
        // index the cursor is at is always in the middle.
        let base_x_offset = SCREEN_WIDTH / 2 - TILE_WIDTH / 2;
        for i in 0..26 {
            let char = AsciiChar(b'A' + i as u8);
            let x = base_x_offset + (i as i16 - self.cursor as i16) * (TILE_WIDTH + TILE_PADDING);
            let y = SCREEN_HEIGHT - TILE_PADDING - TILE_WIDTH;
            let palette_index = self.letter_states[i].palette_index();
            let tile_index = if x < -TILE_WIDTH || x >= SCREEN_WIDTH {
                NULL_TILE * 4 + 1
            } else {
                char.tile_index()
            };

            let obj = ObjAttr::new()
                .size(TileSize::SIZE_16X16)
                .tile(tile_index)
                .palette(palette_index)
                .x(x)
                .y(y);

            let offset = 30 + i;
            OBJ_ATTRS.index(offset).write(obj);
        }

        // Draw the cursor
        let cursor_x = base_x_offset;
        let cursor_y = SCREEN_HEIGHT - TILE_PADDING - TILE_WIDTH * 2 - 2;
        let obj = ObjAttr::new()
            .size(TileSize::SIZE_16X16)
            .tile(27 * 4 + 1)
            .palette(BLACK_PALETTE)
            .x(cursor_x)
            .y(cursor_y);

        OBJ_ATTRS.index(64).write(obj);
    }
}
