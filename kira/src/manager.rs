mod backend;

use std::error::Error;

use backend::Backend;
use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Stream,
};
use flume::Sender;

use crate::command::Command;

pub struct AudioManager<'a> {
	command_sender: Sender<Command<'a>>,
	_stream: Stream,
}

impl<'a> AudioManager<'a> {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let host = cpal::default_host();
		let device = host.default_output_device().unwrap();
		let config = device.default_output_config()?.config();
		let sample_rate = config.sample_rate.0;
		let channels = config.channels;
		let (command_sender, command_receiver) = flume::bounded(10);
		let mut backend = Backend::new(sample_rate, command_receiver);
		let stream = device.build_output_stream(
			&config,
			move |data: &mut [f32], _| {
				for frame in data.chunks_exact_mut(channels as usize) {
					let out = backend.process();
					if channels == 1 {
						frame[0] = (out.left + out.right) / 2.0;
					} else {
						frame[0] = out.left;
						frame[1] = out.right;
					}
				}
			},
			move |_| {},
		)?;
		stream.play()?;
		Ok(Self {
			command_sender,
			_stream: stream,
		})
	}
}
