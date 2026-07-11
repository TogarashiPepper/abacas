//! The symbol struct and related items.

use std::borrow::Cow;

/// Represents a symbol that can be used as an identifier.
#[derive(Debug)]
pub struct Symbol {
	/// The name of this symbol.
	name: Cow<'static, str>,
}

impl Symbol {
	/// Gets the name of this symbol.
	pub const fn name(&self) -> &str {
		match &self.name {
			Cow::Borrowed(borrowed) => borrowed,
			Cow::Owned(owned) => owned.as_str(),
		}
	}
}
