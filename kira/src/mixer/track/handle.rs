use crate::{
	audio_stream::{AudioStream, AudioStreamId},
	command::{sender::CommandSender, MixerCommand, StreamCommand},
	mixer::effect::{Effect, EffectId, EffectSettings},
	AudioResult,
};

use super::TrackId;

pub struct TrackHandle {
	id: TrackId,
	command_sender: CommandSender,
}

impl TrackHandle {
	pub(crate) fn new(id: TrackId, command_sender: CommandSender) -> Self {
		Self { id, command_sender }
	}

	pub fn id(&self) -> TrackId {
		self.id
	}

	pub fn add_effect(
		&mut self,
		effect: impl Effect + 'static,
		settings: EffectSettings,
	) -> AudioResult<EffectId> {
		let effect_id = EffectId::new(self.id);
		self.command_sender
			.push(MixerCommand::AddEffect(self.id, effect_id, Box::new(effect), settings).into())
			.map(|_| effect_id)
	}

	pub fn remove_effect(&mut self, id: EffectId) -> AudioResult<()> {
		self.command_sender
			.push(MixerCommand::RemoveEffect(id).into())
	}

	pub fn add_stream(&mut self, stream: impl AudioStream) -> AudioResult<AudioStreamId> {
		let stream_id = AudioStreamId::new();
		self.command_sender
			.push(StreamCommand::AddStream(stream_id, self.id(), Box::new(stream)).into())
			.map(|()| stream_id)
	}

	pub fn remove_stream(&mut self, id: AudioStreamId) -> AudioResult<()> {
		self.command_sender
			.push(StreamCommand::RemoveStream(id).into())
	}
}
