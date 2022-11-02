// TODO: include the file locations as context

//! Utility span types for tying together tokens, source code, and ast nodes.

/// A span of source code corrseponding to a token (or something).
pub type Span = std::ops::Range<usize>;

/// A pair of (`T`, [`Span`]).
pub type Spanned<T> = (T, Span);

// use std::{fmt, ops::Range};

// /// A span of source code corrseponding to a token (or something).

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub struct Span {
//     src: String,
//     range: (usize, usize),
// }

// impl Span {
//     pub fn new(src: String, range: Range<usize>) -> Self {
//         Self {
//             src,
//             range: (range.start, range.end),
//         }
//     }

//     fn start(&self) -> usize {
//         self.range.0
//     }
//     fn end(&self) -> usize {
//         self.range.1
//     }

//     #[cfg(test)]
//     pub fn empty() -> Self {
//         Self::new(String::new(), 0..0)
//     }

//     pub fn src(&self) -> String {
//         self.src
//     }

//     pub fn range(&self) -> Range<usize> {
//         self.start()..self.end()
//     }

//     pub fn union(self, other: Self) -> Self {
//         assert_eq!(
//             self.src, other.src,
//             "attempted to union spans with different sources"
//         );
//         Self {
//             range: (self.start().min(other.start()), self.end().max(other.end())),
//             ..self
//         }
//     }
// }

// impl fmt::Debug for Span {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}:{:?}", self.src, self.range())
//     }
// }

// impl chumsky::Span for Span {
//     type Context = String;
//     type Offset = usize;

//     fn new(src: String, range: Range<usize>) -> Self {
//         assert!(range.start <= range.end);
//         Self {
//             src,
//             range: (range.start, range.end),
//         }
//     }

//     fn context(&self) -> String {
//         self.src
//     }
//     fn start(&self) -> Self::Offset {
//         self.range.0
//     }
//     fn end(&self) -> Self::Offset {
//         self.range.1
//     }
// }

// /// A pair of (`T`, [`Span`]).
// pub type Spanned<T> = (T, Span);
