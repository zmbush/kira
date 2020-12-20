use crate::{
	util::{lerp, random_float_0_1, random_i32},
	value::AsValue,
};

const SEMITONE_RATIO: f64 = 1.05946309436;

#[derive(Debug, Clone, Copy)]
pub enum Pitch {
	Factor(f64),
	Semitones(i32),
}

impl Pitch {
	pub fn to_factor(self) -> f64 {
		match self {
			Pitch::Factor(factor) => factor,
			Pitch::Semitones(semitones) => SEMITONE_RATIO.powi(semitones),
		}
	}
}

impl AsValue for Pitch {
	fn random_in_range(lower: Self, upper: Self, rng: &mut impl nanorand::RNG) -> Self {
		if let (Self::Semitones(a), Self::Semitones(b)) = (lower, upper) {
			Self::Semitones(random_i32(a, b, rng))
		} else {
			Self::Factor(lerp(
				lower.to_factor(),
				upper.to_factor(),
				random_float_0_1(rng),
			))
		}
	}
}

impl From<f64> for Pitch {
	fn from(factor: f64) -> Self {
		Self::Factor(factor)
	}
}

impl Default for Pitch {
	fn default() -> Self {
		Self::Factor(1.0)
	}
}
