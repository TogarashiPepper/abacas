/// A variable width big integer
struct Integer(Vec<u64>);

impl Integer {
	fn add(&self, rhs: &Self) -> Self {
		let len = self.0.len().max(rhs.0.len());
		let mut result = Vec::with_capacity(len);
		let mut carry = false;

		for i in 0..len {
			let lhs = self.0.get(i).copied().unwrap_or(0);
			let rhs = rhs.0.get(i).copied().unwrap_or(0);

			let (sum1, overflow1) = lhs.overflowing_add(rhs);
			let (final_sum, overflow2) = sum1.overflowing_add(carry as u64);

			carry = overflow1 || overflow2;

			result.push(final_sum);
		}

		if carry {
			result.push(1);
		}

		Integer(result)
	}
}
