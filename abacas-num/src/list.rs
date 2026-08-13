//! The digit type and related items.

use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::SliceIndex;

/// A growable list that can keep a single value on the stack.
#[derive(Debug)]
pub enum List<T> {
	/// A list of values on the heap.
	Heap(Vec<T>),
	/// A single value on the stack.
	Stack(Option<T>),
}

impl<T> List<T> {
	/// Creates a new, empty list.
	pub const fn new() -> Self {
		Self::Stack(None)
	}

	/// Creates a new list from a singleton value.
	pub const fn singleton(value: T) -> Self {
		Self::Stack(Some(value))
	}
}

impl<T> List<T> {
	/// Converts this list into a mutable slice.
	pub const fn as_mut_slice(&mut self) -> &mut [T] {
		match self {
			Self::Heap(heap) => heap.as_mut_slice(),
			Self::Stack(stack) => stack.as_mut_slice(),
		}
	}

	/// Converts this list into a slice.
	pub const fn as_slice(&self) -> &[T] {
		match self {
			Self::Heap(heap) => heap.as_slice(),
			Self::Stack(stack) => stack.as_slice(),
		}
	}

	/// Gets the capacity of this list.
	pub const fn capacity(&self) -> NonZeroUsize {
		match self {
			Self::Heap(heap) => match NonZeroUsize::new(heap.capacity()) {
				None => NonZeroUsize::MIN,
				Some(capacity) => capacity,
			},

			Self::Stack(_) => NonZeroUsize::MIN,
		}
	}

	/// Removes the last value from this list.
	pub fn pop(&mut self) -> Option<T> {
		match self {
			Self::Heap(heap) => heap.pop(),
			Self::Stack(stack) => stack.take(),
		}
	}

	/// Adds a new value to the list.
	pub fn push(&mut self, value: T) {
		match self {
			Self::Heap(heap) => match heap.capacity() {
				0 => *self = Self::Stack(Some(value)),
				_ => heap.push(value),
			},

			Self::Stack(stack) => match stack.take() {
				None => *self = Self::Stack(Some(value)),
				Some(stack) => *self = Self::Heap(vec![stack, value]),
			},
		}
	}
}

impl<T> Clone for List<T>
where
	T: Clone,
{
	fn clone(&self) -> Self {
		match self {
			Self::Heap(heap) => match heap.len() {
				..2 => Self::Stack(heap.first().cloned()),
				2.. => Self::Heap(heap.clone()),
			},

			Self::Stack(stack) => Self::Stack(stack.clone()),
		}
	}
}

impl<T> Default for List<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Deref for List<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl<T> DerefMut for List<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

impl<T, I> Index<I> for List<T>
where
	I: SliceIndex<[T]>,
{
	type Output = I::Output;

	fn index(&self, index: I) -> &Self::Output {
		self.as_slice().index(index)
	}
}

impl<T, I> IndexMut<I> for List<T>
where
	I: SliceIndex<[T]>,
{
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.as_mut_slice().index_mut(index)
	}
}
