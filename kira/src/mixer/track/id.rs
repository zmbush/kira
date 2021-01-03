use std::sync::atomic::AtomicUsize;

use atomic::Ordering;
use bimap::BiMap;

use crate::{AudioError, AudioResult};

use super::TrackHandle;

static NEXT_SUB_TRACK_ID: AtomicUsize = AtomicUsize::new(0);

/**
A unique identifier for a sub-track.

You cannot create this manually - a `SubTrackId` is created
when you create a sub-track with an [`AudioManager`](crate::manager::AudioManager).
*/
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubTrackId {
	index: usize,
}

impl SubTrackId {
	pub(crate) fn new() -> Self {
		let index = NEXT_SUB_TRACK_ID.fetch_add(1, Ordering::Relaxed);
		Self { index }
	}
}

mod private {
	pub trait Sealed {}
}

pub trait TrackIdTrait: private::Sealed + Sized + std::fmt::Debug + Clone + Default {
	fn to_track_id(self, sub_track_names: &BiMap<String, SubTrackId>) -> AudioResult<TrackId>;
}

/// Represents a mixer track.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TrackId {
	/// The main track.
	///
	/// All sub-tracks are sent to the main track as input,
	/// and the output of the main track is what you hear.
	Main,
	/// A sub-track.
	///
	/// Sub-tracks are useful for adjusting the volumes of
	/// and applying effects to certain kinds of sounds.
	/// For example, in a game, you may have one sub-track
	/// for sound effects and another for music.
	Sub(SubTrackId),
}

impl Default for TrackId {
	fn default() -> Self {
		Self::Main
	}
}

impl From<SubTrackId> for TrackId {
	fn from(id: SubTrackId) -> Self {
		Self::Sub(id)
	}
}

impl From<&TrackHandle> for TrackId {
	fn from(handle: &TrackHandle) -> Self {
		handle.id()
	}
}

impl private::Sealed for TrackId {}

impl TrackIdTrait for TrackId {
	fn to_track_id(self, _: &BiMap<String, SubTrackId>) -> AudioResult<TrackId> {
		Ok(self)
	}
}

#[derive(Debug, Clone)]
pub enum TrackLabel {
	Id(TrackId),
	Name(String),
}

impl Default for TrackLabel {
	fn default() -> Self {
		Self::Id(TrackId::Main)
	}
}

impl From<TrackId> for TrackLabel {
	fn from(id: TrackId) -> Self {
		Self::Id(id)
	}
}

impl From<SubTrackId> for TrackLabel {
	fn from(id: SubTrackId) -> Self {
		Self::Id(TrackId::Sub(id))
	}
}

impl From<&TrackHandle> for TrackLabel {
	fn from(handle: &TrackHandle) -> Self {
		Self::Id(handle.id())
	}
}

impl From<String> for TrackLabel {
	fn from(name: String) -> Self {
		Self::Name(name)
	}
}

impl private::Sealed for TrackLabel {}

impl TrackIdTrait for TrackLabel {
	fn to_track_id(self, sub_track_names: &BiMap<String, SubTrackId>) -> AudioResult<TrackId> {
		match self {
			TrackLabel::Id(id) => Ok(id),
			TrackLabel::Name(name) => sub_track_names
				.get_by_left(&name)
				.map(|id| TrackId::Sub(*id))
				.ok_or(AudioError::NoTrackWithName(name)),
		}
	}
}
