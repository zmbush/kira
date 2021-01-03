use bimap::BiMap;
use indexmap::IndexSet;

use crate::AudioResult;

use super::{groups::Groups, GroupId, GroupIdTrait, GroupLabel};

#[derive(Debug, Clone)]
pub struct GroupSet<GroupIdType: GroupIdTrait = GroupLabel>(IndexSet<GroupIdType>);

pub(crate) type InternalGroupSet = GroupSet<GroupId>;

impl GroupSet {
	pub fn new() -> Self {
		Self(IndexSet::new())
	}

	pub fn add(mut self, id: impl Into<GroupLabel>) -> Self {
		self.0.insert(id.into());
		self
	}

	pub fn remove(mut self, id: impl Into<GroupLabel>) -> Self {
		self.0.remove(&id.into());
		self
	}

	pub fn contains(&self, id: impl Into<GroupLabel>) -> bool {
		self.0.contains(&id.into())
	}
}

impl InternalGroupSet {
	pub(crate) fn from_public_group_set(
		group_set: GroupSet,
		group_names: &BiMap<String, GroupId>,
	) -> AudioResult<Self> {
		let mut set = IndexSet::new();
		for label in &group_set.0 {
			set.insert(label.to_group_id(group_names)?);
		}
		Ok(Self(set))
	}

	/// Returns true if one of the groups in the set has a specified
	/// group as an ancestor or is that group itself.
	pub(crate) fn has_ancestor(&self, ancestor: GroupId, all_groups: &Groups) -> bool {
		// make sure the group actually exists
		if all_groups.get(ancestor).is_none() {
			return false;
		}
		// check if any groups in this set are the target group
		for id in &self.0 {
			if *id == ancestor {
				return true;
			}
		}
		// otherwise, recursively check if the target group
		// is an ancestor of any groups in the set
		for id in &self.0 {
			if let Some(group) = all_groups.get(*id) {
				if group.groups().has_ancestor(ancestor, all_groups) {
					return true;
				}
			}
		}
		false
	}
}
