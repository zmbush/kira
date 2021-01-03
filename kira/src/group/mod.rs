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
pub use id::*;
pub use set::GroupSet;

#[derive(Debug, Clone)]
pub(crate) struct Group {
	groups: GroupSet,
}

impl Group {
	pub fn new(groups: GroupSet) -> Self {
		Self { groups }
	}

	pub fn groups(&self) -> &GroupSet {
		&self.groups
	}
}
