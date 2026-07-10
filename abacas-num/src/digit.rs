//! The digit type and related items.

/// Represents a single digit.
pub type Digit = u64;

/// Represents a list of digits with more optimal memory usage.
#[derive(Clone, Debug)]
pub enum Digits {
	/// A list of values on the heap.
	Heap(Vec<Digit>),
	/// A single value on the stack.
	Stack(Option<Digit>),
}
