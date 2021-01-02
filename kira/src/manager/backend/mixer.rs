use bimap::BiMap;
use flume::Sender;
use indexmap::IndexMap;

use crate::{
	command::MixerCommand,
	frame::Frame,
	mixer::{SubTrackId, Track, TrackIndex, TrackSettings},
	parameter::Parameters,
	resource::Resource,
};

pub(crate) struct Mixer {
	main_track: Track,
	sub_tracks: IndexMap<SubTrackId, Track>,
	sub_track_names: BiMap<&'static str, SubTrackId>,
}

impl Mixer {
	pub fn new() -> Self {
		Self {
			main_track: Track::new(TrackSettings::default()),
			/// TODO: preallocate memory
			sub_tracks: IndexMap::new(),
			sub_track_names: BiMap::new(),
		}
	}

	pub fn run_command(&mut self, command: MixerCommand, unloader: &mut Sender<Resource>) {
		match command {
			MixerCommand::AddSubTrack(id, track) => {
				if let Some(name) = track.name() {
					self.sub_track_names.insert(name, id);
				}
				self.sub_tracks.insert(id, track);
			}
			MixerCommand::AddEffect(index, id, effect, settings) => {
				let track = match index {
					TrackIndex::Main => Some(&mut self.main_track),
					TrackIndex::Sub(id) => self.sub_tracks.get_mut(&id),
					TrackIndex::Named(name) => {
						if let Some(id) = self.sub_track_names.get_by_left(&name) {
							self.sub_tracks.get_mut(id)
						} else {
							None
						}
					}
				};
				if let Some(track) = track {
					track.add_effect(id, effect, settings);
				}
			}
			MixerCommand::RemoveSubTrack(id) => {
				if let Some(track) = self.sub_tracks.remove(&id) {
					unloader.try_send(Resource::Track(track)).ok();
					self.sub_track_names.remove_by_right(&id);
				}
			}
			MixerCommand::RemoveEffect(effect_id) => {
				let track = match effect_id.track_index() {
					TrackIndex::Main => Some(&mut self.main_track),
					TrackIndex::Sub(id) => self.sub_tracks.get_mut(&id),
					TrackIndex::Named(name) => {
						if let Some(id) = self.sub_track_names.get_by_left(&name) {
							self.sub_tracks.get_mut(id)
						} else {
							None
						}
					}
				};
				if let Some(track) = track {
					if let Some(effect_slot) = track.remove_effect(effect_id) {
						unloader.try_send(Resource::EffectSlot(effect_slot)).ok();
					}
				}
			}
		}
	}

	pub fn add_input(&mut self, index: TrackIndex, input: Frame) {
		let track = match index {
			TrackIndex::Main => Some(&mut self.main_track),
			TrackIndex::Sub(id) => self.sub_tracks.get_mut(&id),
			TrackIndex::Named(name) => {
				if let Some(id) = self.sub_track_names.get_by_left(&name) {
					self.sub_tracks.get_mut(id)
				} else {
					None
				}
			}
		};
		if let Some(track) = track {
			track.add_input(input);
		}
	}

	pub fn process(&mut self, dt: f64, parameters: &Parameters) -> Frame {
		let mut main_input = Frame::from_mono(0.0);
		for (_, sub_track) in &mut self.sub_tracks {
			main_input += sub_track.process(dt, parameters);
		}
		self.main_track.add_input(main_input);
		self.main_track.process(dt, parameters)
	}
}
