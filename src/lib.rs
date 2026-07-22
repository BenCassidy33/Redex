use std::ops::Range;

pub mod node;

pub const LAMBDA_CHAR: char = 'L';

pub fn find_closing_delim(s: &str, open: char, close: char) -> Result<Range<usize>, usize> {
    let mut first_found = usize::MAX;
    let mut count = 0;

    for (idx, ch) in s.chars().enumerate() {
        if ch == open {
            if first_found == usize::MAX {
                first_found = idx;
            }

            count += 1;
            continue;
        }

        if ch == close {
            count -= 1;

            if count == 0 && first_found != usize::MAX {
                return Ok(first_found..idx);
            }
        }
    }

    Err(count)
}
