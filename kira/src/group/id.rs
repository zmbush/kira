use std::sync::atomic::{AtomicUsize, Ordering};

use super::GroupHandle;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupIndex {
	Id(GroupId),
	Name(&'static str),
}

impl From<GroupId> for GroupIndex {
	fn from(id: GroupId) -> Self {
		Self::Id(id)
	}
}

impl From<&GroupHandle> for GroupIndex {
	fn from(handle: &GroupHandle) -> Self {
		Self::Id(handle.id())
	}
}

impl From<&'static str> for GroupIndex {
	fn from(name: &'static str) -> Self {
		Self::Name(name)
	}
}
