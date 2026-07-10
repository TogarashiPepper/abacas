//! The rational struct and related items.

use crate::integer::Integer;
use crate::natural::Natural;

/// Represents a rational number.
pub struct Rational {
	denom: Natural,
	numer: Integer,
}
