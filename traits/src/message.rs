use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::cmp::Eq;

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PurposeType {
	RedeemSpirit,
	BuyPrimeOriginOfShells,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, TypeInfo)]
pub struct OverlordMessage<AccountId> {
	pub account: AccountId,
	pub purpose: PurposeType,
}
