use std::f32::consts::TAU;

use crate::frame::Frame;

pub struct Backend {
	dt: f32,
	phase: f32,
}

impl Backend {
	pub fn new(sample_rate: u32) -> Self {
		Self {
			dt: 1.0 / sample_rate as f32,
			phase: 0.0,
		}
	}

	pub fn process(&mut self) -> Frame {
		self.phase += 440.0 * self.dt;
		self.phase %= 1.0;
		Frame::from_mono((self.phase * TAU).sin() * 0.25)
	}
}
