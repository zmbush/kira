use indexmap::IndexMap;

use crate::{
	arrangement::{Arrangement, ArrangementId},
	mixer::TrackIndex,
	sound::{Sound, SoundId},
	Frame,
};

/// Useful info about a [`Playable`](Playable) item.
#[derive(Debug, Default, Copy, Clone)]
pub struct Metadata {
	/// How long the sound is musically.
	///
	/// For example, a recording of a 2-bar drum fill
	/// in an echoey cathedral may have 5 seconds of actual
	/// drumming and then 10 seconds of reverberations from
	/// the building. So even though the audio is 15 seconds
	/// long, you might say the music only lasts for 5 seconds.
	///
	/// If set, the semantic duration of the sound will be
	/// used as the default end point when looping the sound.
	pub semantic_duration: Option<f64>,
}

/// Settings for a [`Playable`](Playable) item.
#[derive(Debug, Clone)]
pub struct PlayableSettings {
	/// The track instances of this item will play on by default.
	pub default_track: TrackIndex,
	/// Whether the item should have a "cool off" period after playing
	/// before it can be played again, and if so, the duration
	/// of that cool off period.
	///
	/// This is useful to avoid situations where the same item
	/// is played multiple times at the exact same point in time,
	/// resulting in the item being louder than normal.
	pub cooldown: Option<f64>,
	/// Information about the item.
	pub metadata: Metadata,
}

impl Default for PlayableSettings {
	fn default() -> Self {
		Self {
			default_track: TrackIndex::Main,
			cooldown: Some(0.0001),
			metadata: Metadata::default(),
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Playable {
	Sound(SoundId),
	Arrangement(ArrangementId),
}

impl Playable {
	pub fn duration(&self) -> f64 {
		match self {
			Playable::Sound(id) => id.duration(),
			Playable::Arrangement(id) => id.duration(),
		}
	}

	pub fn default_track_index(&self) -> TrackIndex {
		match self {
			Playable::Sound(id) => id.default_track_index(),
			Playable::Arrangement(id) => id.default_track_index(),
		}
	}

	pub(crate) fn get_frame_at_position(
		&self,
		position: f64,
		sounds: &IndexMap<SoundId, Sound>,
		arrangements: &IndexMap<ArrangementId, Arrangement>,
	) -> Frame {
		match self {
			Playable::Sound(id) => {
				if let Some(sound) = sounds.get(id) {
					sound.get_frame_at_position(position)
				} else {
					Frame::from_mono(0.0)
				}
			}
			Playable::Arrangement(id) => {
				if let Some(arrangement) = arrangements.get(id) {
					arrangement.get_frame_at_position(position, sounds)
				} else {
					Frame::from_mono(0.0)
				}
			}
		}
	}
}

impl From<SoundId> for Playable {
	fn from(id: SoundId) -> Self {
		Self::Sound(id)
	}
}

impl From<ArrangementId> for Playable {
	fn from(id: ArrangementId) -> Self {
		Self::Arrangement(id)
	}
}