use std::hash::Hash;

use indexmap::IndexSet;
use nanorand::RNG;

pub fn lerp(a: f64, b: f64, amount: f64) -> f64 {
	a + (b - a) * amount
}

pub fn inverse_lerp(start: f64, end: f64, point: f64) -> f64 {
	(point - start) / (end - start)
}

pub fn index_set_from_vec<T: Hash + Eq>(v: Vec<T>) -> IndexSet<T> {
	let mut set = IndexSet::new();
	for item in v {
		set.insert(item);
	}
	set
}

pub fn random_i32(lower: i32, upper: i32, rng: &mut impl RNG) -> i32 {
	let range = (upper - lower).abs() as u16;
	let sign = (upper - lower).signum();
	lower + i32::from(rng.generate_range(0, range)) * sign
}

pub fn random_float_0_1(rng: &mut impl RNG) -> f64 {
	f64::from(rng.generate::<u32>()) / f64::from(std::u32::MAX)
}
