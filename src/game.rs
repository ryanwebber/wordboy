use ab_os::{
    input::KeyInput,
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

pub struct Game {
    instance: Instance,
    prev_input: KeyInput,
}

impl Game {
    pub fn new(seed: u16) -> Self {
        Self {
            instance: Instance::new(WordBuffer::from_str("HELLO")),
            prev_input: KeyInput(0),
        }
    }

    pub fn state(&self) -> State {
        let word_guessed = self.instance.last_submitted_guess().map_or(false, |guess| {
            guess.as_slice() == self.instance.word.as_slice()
        });

        if word_guessed {
            State::Completed
        } else if self.instance.finished_guessing {
            State::Failed
        } else {
            State::InProgress
        }
    }

    pub fn update(&mut self, input: KeyInput) {
        if input.a_once(self.prev_input) {
            self.instance.input(Input::Char);
        }

        if input.b_once(self.prev_input) {
            self.instance.input(Input::Delete);
        }

        if input.left_once(self.prev_input) {
            self.instance.input(Input::CursorLeft);
        }

        if input.right_once(self.prev_input) {
            self.instance.input(Input::CursorRight);
        }

        if input.up_once(self.prev_input) {
            self.instance.input(Input::Submit);
        }

        self.prev_input = input;
    }

    pub fn render(&self) {
        self.instance.render();
    }
}

pub enum State {
    Completed,
    Failed,
    InProgress,
}

struct Instance {
    word: WordBuffer,
    guesses: ArrayVec<WordBuffer, TILE_ROW_COUNT>,
    letter_states: [GuessInstance; 27],
    finished_guessing: bool,
    cursor: u8,
}

impl Instance {
    fn new(word: WordBuffer) -> Self {
        Self {
            word,
            guesses: {
                let mut guesses = ArrayVec::new();
                guesses.push(WordBuffer::EMPTY);
                guesses
            },
            letter_states: [GuessInstance::Grey; 27],
            finished_guessing: false,
            cursor: 0,
        }
    }

    fn last_submitted_guess(&self) -> Option<&WordBuffer> {
        if self.guesses.len() > 1 {
            self.guesses.nth(self.guesses.len() - 2)
        } else {
            None
        }
    }

    fn current_guess(&self) -> &WordBuffer {
        self.guesses.last().unwrap()
    }

    fn input(&mut self, input: Input) {
        match input {
            Input::Char => {
                let current_guess = self.guesses.last_mut().unwrap();
                if current_guess.is_full() {
                    return;
                }

                let letter = AsciiChar(b'A' + self.cursor);
                current_guess.push(letter);
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
                            .maybe_upgrade(GuessInstance::Green);
                    } else if self.word.as_slice().contains(c) {
                        self.letter_states[c.letter_index() as usize]
                            .maybe_upgrade(GuessInstance::Yellow);
                    } else {
                        self.letter_states[c.letter_index() as usize] = GuessInstance::Black;
                    }
                }

                // Add a new guess if the current one is full
                self.finished_guessing = self.guesses.try_push(WordBuffer::EMPTY).is_err();
            }
        }
    }

    fn render(&self) {
        fn draw_guessed_tile(
            char: AsciiChar,
            row: usize,
            col: usize,
            palette: u16,
            allocator: &mut ObjAttrAllocator,
        ) {
            let x = ROW_OFFSET + (col as i16) * (TILE_WIDTH + TILE_PADDING);
            let y = TILE_PADDING + (row as i16) * (TILE_WIDTH + TILE_PADDING);

            let obj = ObjAttr::new()
                .size(TileSize::SIZE_16X16)
                .tile(char.tile_index())
                .palette(palette)
                .x(x)
                .y(y);

            allocator.allocate_and_write(obj);
        }

        let mut attr_allocator = ObjAttrAllocator::new();

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

                draw_guessed_tile(*char, row, col, palette_index, &mut attr_allocator);
            }
        }

        let unused_guess_count = self.guesses.capacity() - self.guesses.len();
        for i in 0..unused_guess_count {
            for j in 0..5 {
                draw_guessed_tile(
                    AsciiChar::NULL,
                    i + self.guesses.len(),
                    j,
                    BLACK_PALETTE,
                    &mut attr_allocator,
                );
            }
        }

        // Render the keyboard. The cursor is always in the middle,
        // so we need to adjust the position of the tiles so whatever
        // index the cursor is at is always in the middle.
        let base_x_offset = SCREEN_WIDTH / 2 - TILE_WIDTH / 2;
        for i in 0..26 {
            let char = AsciiChar(b'A' + i as u8);
            let x = base_x_offset + (i as i16 - self.cursor as i16) * (TILE_WIDTH + TILE_PADDING);
            let y = SCREEN_HEIGHT - TILE_WIDTH - 12;
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

            attr_allocator.allocate_and_write(obj);
        }

        // Draw the cursor
        let cursor_x = base_x_offset;
        let cursor_y = SCREEN_HEIGHT - TILE_PADDING - 6;
        let obj = ObjAttr::new()
            .size(TileSize::SIZE_16X16)
            .tile(27 * 4 + 1)
            .palette(BLACK_PALETTE)
            .x(cursor_x)
            .y(cursor_y);

        attr_allocator.allocate_and_write(obj);
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GuessInstance {
    Grey = 0,
    Yellow = 1,
    Green = 2,
    Black = 3,
}

impl GuessInstance {
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

struct ObjAttrAllocator(u16);

impl ObjAttrAllocator {
    fn new() -> Self {
        Self(0)
    }

    fn allocate(&mut self) -> usize {
        let index = self.0;
        self.0 += 1;
        index as usize
    }

    fn allocate_and_write(&mut self, attr: ObjAttr) {
        let index = self.allocate();
        OBJ_ATTRS.index(index).write(attr);
    }
}
