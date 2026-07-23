use std::fmt::Display;

use anyhow::bail;
use derive_more::{Constructor, Eq};

use crate::utils::find_closing_delim;

#[derive(Debug, PartialEq, Constructor, Clone, Hash, Eq)]
pub struct Variable {
    pub(crate) ident: String,
    pub(crate) subtext: Option<String>,
}

impl TryFrom<&str> for Variable {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Variable::parse_str(value).map_err(|_| ())
    }
}

impl Variable {
    pub fn len(&self) -> usize {
        self.ident.len()
            + if let Some(ref st) = self.subtext {
                if st.len() != 1 {
                    st.len() + 3
                } else {
                    st.len() + 1
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
        } else if let Some((_, ch)) = chars.next() {
            ident = ch.to_string()
        } else {
            bail!("Invalid variable input: {:?}", s)
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

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.subtext {
            Some(sub) => {
                if sub.len() > 1 {
                    write!(f, "{}_{{{}}}", self.ident, sub)
                } else {
                    write!(f, "{}_{}", self.ident, sub)
                }
            }

            None => write!(f, "{}", self.ident),
        }
    }
}
