#[cfg(feature = "serde_support")]
pub(crate) mod serde;

use indexmap::IndexSet;

use super::{groups::Groups, GroupId, GroupIndex};

#[derive(Debug, Clone)]
pub struct GroupSet {
	set: IndexSet<GroupIndex>,
}

impl GroupSet {
	pub fn new() -> Self {
		Self {
			set: IndexSet::new(),
		}
	}

	pub fn add(mut self, index: impl Into<GroupIndex>) -> Self {
		self.set.insert(index.into());
		self
	}

	pub fn remove(mut self, index: impl Into<GroupIndex>) -> Self {
		self.set.remove(&index.into());
		self
	}

	pub fn contains(&self, index: impl Into<GroupIndex>) -> bool {
		self.set.contains(&index.into())
	}

	pub(crate) fn is_in_group(&self, target_id: GroupId, groups: &Groups) -> bool {
		// make sure the group actually exists
		if groups.get(target_id.into()).is_none() {
			return false;
		}
		// check if any of the groups in this set are the target group
		for index in &self.set {
			if let Some((id, _)) = groups.get(*index) {
				if id == target_id {
					return true;
				}
			}
		}
		// otherwise, recursively check if any of the groups in this set
		// are themselves in the target group
		for index in &self.set {
			if let Some((_, group)) = groups.get(*index) {
				if group.groups().is_in_group(target_id, groups) {
					return true;
				}
			}
		}
		false
	}
}
