use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{frame::Frame, sound::Sound};

static NEXT_INSTANCE_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceId(usize);

impl InstanceId {
	pub(crate) fn new() -> Self {
		Self(NEXT_INSTANCE_INDEX.fetch_add(1, Ordering::Relaxed))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceState {
	Playing,
	Stopped,
}

pub(crate) struct Instance<'a> {
	sound: &'a Sound,
	position: f64,
	state: InstanceState,
}

impl<'a> Instance<'a> {
	pub fn new(sound: &'a Sound) -> Self {
		Self {
			sound,
			position: 0.0,
			state: InstanceState::Playing,
		}
	}

	pub fn process(&mut self, dt: f64) -> Frame {
		let frame = self.sound.frame_at_position(self.position);
		self.position += dt;
		if self.position >= self.sound.duration() {
			self.state = InstanceState::Stopped;
		}
		frame
	}
}
