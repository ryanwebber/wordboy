use wordboy::{
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
const KBD_ANIMATION_SPEED: u16 = 4;

const GREEN_PALETTE: u16 = 1;
const YELLOW_PALETTE: u16 = 2;
const GREY_PALETTE: u16 = 3;
const BLACK_PALETTE: u16 = 4;
const POPUP_WIN_PALETTE: u16 = 5;
const POPUP_LOSE_PALETTE: u16 = 6;

const NULL_TILE: u16 = 47;

pub struct SplashScreen(u16);

impl SplashScreen {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn update(&mut self) {
        self.0 += 1;
    }

    pub fn render(&self) {
        fn draw_text(
            text: &str,
            y_offset: i16,
            palette_offset: u16,
            attr_allocator: &mut ObjAttrAllocator,
        ) {
            let len = text.len() as i16;
            let x_left = (SCREEN_WIDTH - ((len * TILE_WIDTH) + (TILE_PADDING * (len - 1)))) / 2;
            for (i, letter) in text.chars().enumerate() {
                let x = x_left + (i as i16) * (TILE_WIDTH + TILE_PADDING);
                let y = y_offset;

                let obj = ObjAttr::new()
                    .size(TileSize::SIZE_16X16)
                    .tile(AsciiChar::from_u8(letter as u8).tile_index())
                    .palette((palette_offset + i as u16) % 3 + 1)
                    .x(x)
                    .y(y);

                attr_allocator.allocate_and_write(obj);
            }
        }

        let tick = self.0 / 32;
        let mut attr_allocator = ObjAttrAllocator::new();

        draw_text("WORD", 40, tick, &mut attr_allocator);

        draw_text(
            "BOY",
            40 + TILE_WIDTH + TILE_PADDING,
            tick + 4,
            &mut attr_allocator,
        );

        // Start button
        let start_off_x = SCREEN_WIDTH / 2 - TILE_WIDTH;
        let start_off_y = SCREEN_HEIGHT - TILE_WIDTH - 24;
        for i in 0..2 {
            let obj = ObjAttr::new()
                .size(TileSize::SIZE_16X16)
                .tile((30 + i) * 4 + 1)
                .palette(GREY_PALETTE)
                .x(start_off_x + (i as i16) * TILE_WIDTH)
                .y(start_off_y);

            attr_allocator.allocate_and_write(obj);
        }
    }
}

pub struct Game {
    instance: Instance,
    prev_input: KeyInput,
    tick: u16,
}

impl Game {
    pub fn new(seed: u16) -> Self {
        let word = dictionary::random_word(seed as usize);
        Self {
            instance: Instance::new(word),
            prev_input: KeyInput(0),
            tick: 0,
        }
    }

    pub fn state(&self) -> State {
        self.instance.state()
    }

    pub fn update(&mut self, input: KeyInput) {
        self.tick += 1;

        if self.instance.keyboard_anim_offset != 0 {
            // Diminish the keyboard animation offset towards zero
            let sign = self.instance.keyboard_anim_offset.signum();
            let abs_offset = self.instance.keyboard_anim_offset.abs();
            let max_diminish = abs_offset.min(KBD_ANIMATION_SPEED as i16);
            self.instance.keyboard_anim_offset -= sign * max_diminish;
        }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    keyboard_anim_offset: i16,
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
            keyboard_anim_offset: 0,
            finished_guessing: false,
            cursor: 0,
        }
    }

    pub fn state(&self) -> State {
        let last_submitted_guess = if self.guesses.len() > 1 {
            let last_guess_index = if self.finished_guessing {
                self.guesses.len() - 1
            } else {
                self.guesses.len() - 2
            };

            self.guesses.nth(last_guess_index)
        } else {
            None
        };

        let word_guessed =
            last_submitted_guess.map_or(false, |guess| guess.as_slice() == self.word.as_slice());

        if word_guessed {
            State::Completed
        } else if self.finished_guessing {
            State::Failed
        } else {
            State::InProgress
        }
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
                    self.keyboard_anim_offset = -(TILE_WIDTH + TILE_PADDING);
                }
            }
            Input::CursorRight => {
                if self.cursor < 25 {
                    self.cursor += 1;
                    self.keyboard_anim_offset = TILE_WIDTH + TILE_PADDING;
                }
            }
            Input::Delete => {
                let current_guess = self.guesses.last_mut().unwrap();
                current_guess.pop();
            }
            Input::Submit => {
                let current_guess = self.guesses.last_mut().unwrap();
                if !current_guess.is_full() {
                    return;
                }

                // Check if the guess is valid or not
                if !dictionary::is_valid_guess(&current_guess) {
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

        // If the game is over, draw the finish screen
        let state = self.state();
        if state != State::InProgress {
            const POPUP_ROWS: [[u16; 7]; 3] = [
                [32, 33, 33, 33, 33, 33, 34],
                [40, 00, 00, 00, 00, 00, 42],
                [48, 49, 49, 49, 49, 49, 50],
            ];

            const POPUP_WIDTH: i16 = 7 * TILE_WIDTH;
            const POPUP_X_OFFSET: i16 = (SCREEN_WIDTH - POPUP_WIDTH) / 2;
            const POPUP_Y_OFFSET: i16 = (SCREEN_HEIGHT - 3 * TILE_WIDTH) / 2;

            let palette = if state == State::Completed {
                POPUP_WIN_PALETTE
            } else {
                POPUP_LOSE_PALETTE
            };

            for (r, row) in POPUP_ROWS.iter().enumerate() {
                for (c, tile) in row.iter().enumerate() {
                    let x = POPUP_X_OFFSET + (c as i16) * TILE_WIDTH;
                    let y = POPUP_Y_OFFSET + (r as i16) * TILE_WIDTH;

                    let obj = if *tile == 0 {
                        let letter = self.word.as_slice()[c - 1].tile_index();
                        ObjAttr::new()
                            .size(TileSize::SIZE_16X16)
                            .tile(letter)
                            .palette(palette)
                            .x(x)
                            .y(y)
                    } else {
                        ObjAttr::new()
                            .size(TileSize::SIZE_16X16)
                            .tile(*tile * 4 + 1)
                            .palette(palette)
                            .x(x)
                            .y(y)
                    };

                    attr_allocator.allocate_and_write(obj);
                }
            }
        }

        // Draw the main grid
        for (row, word) in self.guesses.iter().enumerate() {
            for (col, char) in word.as_slice().iter().enumerate() {
                let palette_index = if *char == AsciiChar::NULL
                    || (row == self.guesses.len() - 1 && !self.finished_guessing)
                {
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
        let base_x_offset = SCREEN_WIDTH / 2 - TILE_WIDTH / 2 + self.keyboard_anim_offset;
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
        let cursor_x = SCREEN_WIDTH / 2 - TILE_WIDTH / 2;
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
