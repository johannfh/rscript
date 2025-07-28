use derive_more::Display;

#[derive(Debug, PartialEq, Clone, Copy, Default, Display)]
#[display("{start}-{end}")]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn combine(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.min(other.end),
        }
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(value: std::ops::Range<usize>) -> Self {
        Span {
            start: value.start,
            end: value.end,
        }
    }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}
