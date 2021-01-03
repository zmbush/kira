mod handle;
mod id;

pub use handle::TrackHandle;
pub use id::*;

use indexmap::IndexMap;

use crate::{frame::Frame, parameter::Parameters};

use super::{
	effect::{Effect, EffectId, EffectSettings},
	effect_slot::EffectSlot,
};

/// Settings for a mixer track.
#[derive(Debug, Copy, Clone)]
pub struct TrackSettings {
	/// The volume of the track.
	pub volume: f64,
}

impl Default for TrackSettings {
	fn default() -> Self {
		Self { volume: 1.0 }
	}
}

#[derive(Debug)]
pub(crate) struct Track {
	volume: f64,
	effect_slots: IndexMap<EffectId, EffectSlot>,
	input: Frame,
}

impl Track {
	pub fn new(settings: TrackSettings) -> Self {
		Self {
			volume: settings.volume,
			effect_slots: IndexMap::new(),
			input: Frame::from_mono(0.0),
		}
	}

	pub fn add_effect(&mut self, id: EffectId, effect: Box<dyn Effect>, settings: EffectSettings) {
		self.effect_slots
			.insert(id, EffectSlot::new(effect, settings));
	}

	pub fn remove_effect(&mut self, id: EffectId) -> Option<EffectSlot> {
		self.effect_slots.remove(&id)
	}

	pub fn add_input(&mut self, input: Frame) {
		self.input += input;
	}

	pub fn process(&mut self, dt: f64, parameters: &Parameters) -> Frame {
		let mut input = self.input;
		self.input = Frame::from_mono(0.0);
		for (_, effect_slot) in &mut self.effect_slots {
			input = effect_slot.process(dt, input, parameters);
		}
		input * (self.volume as f32)
	}
}
