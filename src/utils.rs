pub fn find_closing_delim(s: &str, open: char, close: char) -> Result<std::ops::Range<usize>, usize> {
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

pub fn group_by_delim(mut s: &str, open: char, close: char) -> Result<Vec<&str>, usize> {
    let mut groups = Vec::new();

    loop {
        let Some(start) = s.find(open) else {
            if groups.is_empty() {
                return Ok(vec![s]);
            } else {
                if !s.is_empty() {
                    groups.push(s);
                }

                return Ok(groups);
            }
        };

        if !&s[..start].is_empty() {
            groups.push(&s[..start]);
        }

        let closing = find_closing_delim(&s[start..], open, close)?;
        groups.push(&s[start + 1..closing.end + start]);

        s = &s[closing.end + start + 1..];
    }
}
