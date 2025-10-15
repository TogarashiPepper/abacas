use rug::Integer as RugInteger;
use rug::float::MiniFloat;

pub enum Number {
	Natural(Natural),
	Integer(Integer),
	Real(Real),
}

pub struct Natural {
	internal: RugInteger,
}

impl Natural {
	pub fn new(n: usize) -> Self {
		Self {
			internal: RugInteger::from(n),
		}
	}
}

pub struct Integer {
	internal: RugInteger,
}

impl Integer {
	pub fn new(n: isize) -> Self {
		Self {
			internal: RugInteger::from(n),
		}
	}
}

pub struct Real {
	internal: MiniFloat,
}

impl Real {
	pub fn new(n: f64) -> Self {
		Self {
			internal: MiniFloat::from(n),
		}
	}
}
