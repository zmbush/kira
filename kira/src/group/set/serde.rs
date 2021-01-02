use serde::{
	de::{SeqAccess, Visitor},
	Serializer,
};

use super::GroupSet;

#[cfg(feature = "serde_support")]
pub(crate) fn serialize<S: Serializer>(
	group_set: &GroupSet,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	serializer.collect_seq(group_set.set.iter())
}

struct StringSeqVisitor;

impl<'de> Visitor<'de> for StringSeqVisitor {
	type Value = Vec<&'static str>;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a sequence of strings")
	}

	fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let mut group_names = vec![];
		while let Some(item) = seq.next_element::<&'static str>()? {
			group_names.push(item);
		}
		Ok(group_names)
	}
}
