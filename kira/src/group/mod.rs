//! Provides an interface for controlling multiple instances
//! and sequences at a time.
//!
//! Groups can be created with [`AudioManager::add_group`](crate::manager::AudioManager::add_group).
//! [`Sound`](crate::sound::Sound)s, [`Arrangement`](crate::arrangement::Arrangement)s
//! and [`Sequence`](crate::sequence::Sequence)s can be assigned
//! to any number of groups when they're created.
//! Groups themselves can also be assigned to groups.
//!
//! [`pause_group`](crate::manager::AudioManager::pause_group),
//! [`resume_group`](crate::manager::AudioManager::resume_group), and
//! [`stop_group`](crate::manager::AudioManager::stop_group) will
//! affect all instances that have the specified group anywhere in
//! their ancestry.

pub(crate) mod groups;
mod handle;
mod id;
mod set;

pub use handle::GroupHandle;
pub use id::{GroupId, GroupIndex};
pub use set::GroupSet;

pub struct GroupSettings {
	pub name: Option<&'static str>,
	pub groups: GroupSet,
}

impl Default for GroupSettings {
	fn default() -> Self {
		Self {
			name: None,
			groups: GroupSet::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Group {
	name: Option<&'static str>,
	groups: GroupSet,
}

impl Group {
	pub fn new(settings: GroupSettings) -> Self {
		Self {
			name: settings.name,
			groups: settings.groups,
		}
	}

	pub fn name(&self) -> Option<&'static str> {
		self.name
	}

	pub fn groups(&self) -> &GroupSet {
		&self.groups
	}
}
