use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Debug, Clone, Copy, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CareerType {
	HackerWizard,
	HardwareDruid,
	RoboWarrior,
	TradeNegotiator,
	Web3Monk,
}

impl CareerType {
	pub fn from_u8(num: u8) -> Option<CareerType> {
		match num {
			0u8 => Some(CareerType::HackerWizard),
			1u8 => Some(CareerType::HardwareDruid),
			2u8 => Some(CareerType::RoboWarrior),
			3u8 => Some(CareerType::TradeNegotiator),
			4u8 => Some(CareerType::Web3Monk),
			_ => None,
		}
	}
}
