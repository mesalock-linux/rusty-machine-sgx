extern crate num as libnum;

pub mod linalg {

	pub trait Metric<T> {
		fn norm(&self) -> T;
	}

	pub mod matrix;
	pub mod vector;
	pub mod utils;
	pub mod macros;
}

pub mod learning {
	pub mod lin_reg;

	pub trait Model<T,U> {
		fn predict(&self, data:T) -> U;

		fn train(&mut self, data:T, value:U);
	}
}