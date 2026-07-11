//! Common operations not included in the standard library.

/// The inversion operation.
pub trait Inv {
	/// The output after the inversion.
	type Output;

	/// Performs the inversion.
	fn inv(self) -> Self::Output;
}

/// The in-place inversion operation.
pub trait InvAssign {
	/// Performs the inversion in-place.
	fn inv_assign(&mut self);
}

/// The in-place negation operation.
pub trait NegAssign {
	/// Performs the negation in-place.
	fn neg_assign(&mut self);
}

/// The in-place bitwise inversion operation.
pub trait NotAssign {
	/// Performs the bitwise inversion in-place.
	fn not_assign(&mut self);
}

/// The exponentiation operation.
pub trait Pow<Rhs = Self> {
	/// The output after the exponentiation.
	type Output;

	/// Performs the exponentiation.
	fn pow(self, rhs: Rhs) -> Self::Output;
}

/// The in-place exponentiation operation.
pub trait PowAssign<Rhs = Self> {
	/// Performs the exponentiation in-place.
	fn pow_assign(&mut self, rhs: Rhs);
}
