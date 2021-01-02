use bimap::BiMap;
use indexmap::IndexMap;

use crate::command::GroupCommand;

use super::{Group, GroupId, GroupIndex};

pub(crate) struct Groups {
	groups: IndexMap<GroupId, Group>,
	group_names: BiMap<&'static str, GroupId>,
}

impl Groups {
	pub fn new(capacity: usize) -> Self {
		Self {
			groups: IndexMap::with_capacity(capacity),
			group_names: BiMap::with_capacity(capacity),
		}
	}

	pub fn get(&self, index: GroupIndex) -> Option<(GroupId, &Group)> {
		match index {
			GroupIndex::Id(id) => self.groups.get(&id).map(|group| (id, group)),
			GroupIndex::Name(name) => self
				.group_names
				.get_by_left(&name)
				.map(|id| self.groups.get(id).map(|group| (*id, group)))
				.flatten(),
		}
	}

	pub fn run_command(&mut self, command: GroupCommand) -> Option<Group> {
		match command {
			GroupCommand::AddGroup(id, group) => {
				self.groups.insert(id, group);
			}
			GroupCommand::RemoveGroup(id) => {
				return self.groups.remove(&id);
			}
		}
		None
	}
}
