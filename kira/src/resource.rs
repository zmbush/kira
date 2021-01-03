use crate::{
	arrangement::Arrangement,
	audio_stream::AudioStream,
	group::Group,
	metronome::Metronome,
	mixer::{effect_slot::EffectSlot, Track},
	sequence::SequenceInstance,
	sound::InternalSound,
};

pub(crate) enum Resource {
	Sound(InternalSound),
	Arrangement(Arrangement),
	SequenceInstance(SequenceInstance),
	Track(Track),
	EffectSlot(EffectSlot),
	Group(Group),
	Stream(Box<dyn AudioStream>),
	Metronome(Metronome),
}
