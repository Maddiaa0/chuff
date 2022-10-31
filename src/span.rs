// TODO: include the file locations as context

//! Utility span types for tying together tokens, source code, and ast nodes.

/// A span of source code corrseponding to a token (or something).
pub type Span = std::ops::Range<usize>;

/// A pair of (`T`, [`Span`]).
pub type Spanned<T> = (T, Span);
