use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Debug, Clone, Copy, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CareerType {
	HardwareDruid,
	RoboWarrior,
	TradeNegotiator,
	HackerWizard,
	Web3Monk,
}
