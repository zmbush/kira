use std::sync::atomic::AtomicUsize;

use atomic::Ordering;
use bimap::BiMap;

use crate::{AudioError, AudioResult};

use super::GroupHandle;

mod private {
	pub trait Sealed {}
}

pub trait GroupIdTrait: private::Sealed + Sized + std::fmt::Debug + Clone {
	fn to_group_id(self, group_names: &BiMap<String, GroupId>) -> AudioResult<GroupId>;
}

static NEXT_GROUP_INDEX: AtomicUsize = AtomicUsize::new(0);

/**
A unique identifier for a group.

You cannot create this manually - a group ID is created
when you create a group with an [`AudioManager`](crate::manager::AudioManager).
*/
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GroupId {
	index: usize,
}

impl GroupId {
	pub(crate) fn new() -> Self {
		let index = NEXT_GROUP_INDEX.fetch_add(1, Ordering::Relaxed);
		Self { index }
	}
}

impl From<&GroupHandle> for GroupId {
	fn from(handle: &GroupHandle) -> Self {
		handle.id()
	}
}

impl private::Sealed for GroupId {}

impl GroupIdTrait for GroupId {
	fn to_group_id(self, _: &BiMap<String, GroupId>) -> AudioResult<GroupId> {
		Ok(self)
	}
}

#[derive(Debug, Clone)]
pub enum GroupLabel {
	Id(GroupId),
	Name(String),
}

impl From<GroupId> for GroupLabel {
	fn from(id: GroupId) -> Self {
		Self::Id(id)
	}
}

impl From<&GroupHandle> for GroupLabel {
	fn from(handle: &GroupHandle) -> Self {
		Self::Id(handle.id())
	}
}

impl From<String> for GroupLabel {
	fn from(name: String) -> Self {
		Self::Name(name)
	}
}

impl From<&str> for GroupLabel {
	fn from(name: &str) -> Self {
		Self::Name(name.into())
	}
}

impl private::Sealed for GroupLabel {}

impl GroupIdTrait for GroupLabel {
	fn to_group_id(self, group_names: &BiMap<String, GroupId>) -> AudioResult<GroupId> {
		match self {
			GroupLabel::Id(id) => Ok(id),
			GroupLabel::Name(name) => group_names
				.get_by_left(&name)
				.cloned()
				.ok_or(AudioError::NoGroupWithName(name)),
		}
	}
}
