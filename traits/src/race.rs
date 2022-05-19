use crate::race::RaceType::Pandroid;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Debug, Clone, Copy, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RaceType {
	Cyborg,
	AISpectre,
	XGene,
	Pandroid,
}

impl RaceType {
	pub fn from_u8(num: u8) -> Option<RaceType> {
		match num {
			0u8 => Some(RaceType::Cyborg),
			1u8 => Some(RaceType::AISpectre),
			2u8 => Some(RaceType::XGene),
			3u8 => Some(RaceType::XGene),
			_ => None,
		}
	}
}
