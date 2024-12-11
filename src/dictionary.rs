use crate::utils::{AsciiChar, WordBuffer};

pub fn valid_guess(word: &WordBuffer) -> bool {
    if word.as_slice().iter().any(|&c| c == AsciiChar::NULL) {
        return false;
    }

    // TODO: Fix this
    !word.as_slice().contains(&AsciiChar(b'Z'))
}
