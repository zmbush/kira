use crate::{
	util::{lerp, random_float_0_1},
	value::AsValue,
};

/// Represents a tempo, or speed, of some music (in beats per minute).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tempo(pub f64);

impl Tempo {
	/// Converts a number of beats at this tempo to a length
	/// of time in seconds.
	pub fn beats_to_seconds(&self, beats: f64) -> f64 {
		(60.0 / self.0) * beats
	}
}

impl AsValue for Tempo {
	fn random_in_range(lower: Self, upper: Self, rng: &mut impl nanorand::RNG) -> Self {
		Self(lerp(lower.0, upper.0, random_float_0_1(rng)))
	}
}

impl From<f64> for Tempo {
	fn from(bpm: f64) -> Self {
		Self(bpm)
	}
}
