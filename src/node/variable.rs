use crate::find_closing_delim;

#[derive(Debug)]
pub struct Variable {
    pub(crate) ident: String,
    pub(crate) subtext: Option<String>,
}

impl Variable {
    pub fn len(&self) -> usize {
        self.ident.len()
            + if let Some(ref st) = self.subtext {
                if st.len() != 1 {
                    st.len() + 2
                } else {
                    st.len()
                }
            } else {
                0
            }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn parse_str(s: &str) -> anyhow::Result<Variable> {
        let s = s.trim();
        let mut chars = s.chars().enumerate().peekable();
        let mut ident = String::new();
        let mut subtext = None;

        if chars.peek().is_some_and(|c| c.1.is_uppercase()) {
            while let Some((_, ch)) = chars.peek()
                && ch.is_uppercase()
                && *ch != '_'
                && !ch.is_whitespace()
            {
                ident.push(chars.next().unwrap().1);
            }
        } else {
            while let Some((_, ch)) = chars.peek()
                && *ch != '_'
                && !ch.is_whitespace()
            {
                ident.push(chars.next().unwrap().1)
            }
        }

        if chars.peek().is_some_and(|(_, c)| *c == '_') {
            let _ = chars.next();

            if let Some((idx, ch)) = chars.next() {
                if ch == '{' {
                    let range = find_closing_delim(&s[idx..], '{', '}').unwrap();
                    subtext = Some(s[range.start + idx + 1..range.end + idx].to_string());
                } else {
                    subtext = Some(ch.to_string())
                }
            }
        }

        Ok(Variable { ident, subtext })
    }
}
