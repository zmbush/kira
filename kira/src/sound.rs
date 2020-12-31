use crate::frame::Frame;

pub struct Sound {
	frames: Vec<Frame>,
	sample_rate: u32,
}

impl Sound {
	/// Creates a new sound from raw sample data.
	pub fn from_frames(sample_rate: u32, frames: Vec<Frame>) -> Self {
		Self {
			sample_rate,
			frames,
		}
	}

	/// Gets the frame of this sound at an arbitrary time
	/// in seconds, interpolating between samples if necessary.
	pub fn get_frame_at_position(&self, position: f64) -> Frame {
		let sample_position = self.sample_rate as f64 * position;
		let x = (sample_position % 1.0) as f32;
		let current_sample_index = sample_position as usize;
		let y0 = if current_sample_index == 0 {
			Frame::from_mono(0.0)
		} else {
			*self
				.frames
				.get(current_sample_index - 1)
				.unwrap_or(&Frame::from_mono(0.0))
		};
		let y1 = *self
			.frames
			.get(current_sample_index)
			.unwrap_or(&Frame::from_mono(0.0));
		let y2 = *self
			.frames
			.get(current_sample_index + 1)
			.unwrap_or(&Frame::from_mono(0.0));
		let y3 = *self
			.frames
			.get(current_sample_index + 2)
			.unwrap_or(&Frame::from_mono(0.0));
		let c0 = y1;
		let c1 = (y2 - y0) * 0.5;
		let c2 = y0 - y1 * 2.5 + y2 * 2.0 - y3 * 0.5;
		let c3 = (y3 - y0) * 0.5 + (y1 - y2) * 1.5;
		((c3 * x + c2) * x + c1) * x + c0
	}
}
