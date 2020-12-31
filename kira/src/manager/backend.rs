use flume::Receiver;
use indexmap::IndexMap;

use crate::{
	command::Command,
	frame::Frame,
	instance::{Instance, InstanceId},
};

const NUM_INSTANCES: usize = 100;

pub struct Backend<'a> {
	dt: f64,
	instances: IndexMap<InstanceId, Instance<'a>>,
	command_receiver: Receiver<Command<'a>>,
}

impl<'a> Backend<'a> {
	pub fn new(sample_rate: u32, command_receiver: Receiver<Command<'a>>) -> Self {
		Self {
			dt: 1.0 / sample_rate as f64,
			instances: IndexMap::with_capacity(NUM_INSTANCES),
			command_receiver,
		}
	}

	pub fn process(&mut self) -> Frame {
		for command in self.command_receiver.try_iter() {
			match command {
				Command::StartInstance(id, instance) => {
					self.instances.insert(id, instance);
				}
			}
		}

		let mut out = Frame::from_mono(0.0);
		for (_, instance) in &mut self.instances {
			out += instance.process(self.dt);
		}
		out
	}
}
