use crate::instance::{Instance, InstanceId};

pub(crate) enum Command<'a> {
	StartInstance(InstanceId, Instance<'a>),
}
