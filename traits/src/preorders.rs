use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::cmp::Eq;

use crate::{career::CareerType, race::RaceType};
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PreorderStatus {
	Pending,
	Chosen,
	NotChosen,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PreorderInfo<AccountId, BoundedString> {
	/// Account owner of the Origin of Shell preorder
	pub owner: AccountId,
	/// Race type of the preorder
	pub race: RaceType,
	/// Career type of the preorder
	pub career: CareerType,
	/// Metadata of the owner
	pub metadata: BoundedString,
	/// Preorder status
	pub preorder_status: PreorderStatus,
}
