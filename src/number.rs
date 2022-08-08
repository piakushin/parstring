use crate::error::Error;

/// Wrapper over `isize` type.
#[derive(Debug, Clone, Copy)]
pub struct Number(pub isize);

impl Number {
    /// Performes a parsing from string.
    fn try_from(s: &str) -> Result<Self, Error> {
        s.parse().map(Self).map_err(Error::Parsing)
    }
}

/// Stack of `char`s (string) to perform parsing.
pub struct NumberStack {
    inner: String,
}

impl NumberStack {
    /// Creates empty stack.
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    /// Pushes char on top of the stack.
    pub fn push(&mut self, ch: char) {
        self.inner.push(ch);
    }

    /// Performs parsing of inner buffer and clears it after.
    pub fn finalize(&mut self) -> Result<Option<Number>, Error> {
        let maybe_num = if self.inner.is_empty() {
            None
        } else {
            let res_num = Number::try_from(&self.inner)?;
            self.inner.clear();
            Some(res_num)
        };
        Ok(maybe_num)
    }
}
