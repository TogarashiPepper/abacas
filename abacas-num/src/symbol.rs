//! The symbol struct and related items.

use std::borrow::Cow;

/// Represents a symbol that can be used as an identifier.
pub struct Symbol(Cow<'static, str>);
