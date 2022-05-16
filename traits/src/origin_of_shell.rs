use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

/// Origin of Shell Types of Prime, Magic & Legendary
#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum OriginOfShellType {
	Prime,
	Magic,
	Legendary,
}

pub trait OriginOfShell<AccountId, BlockNumber> {
	/// When a user initiates the incubation process, this function will set the start time for the
	/// incubation process.
	fn set_start_incubation_time(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
	) -> Result<BlockNumber, DispatchError>;
	/// Get the `incubation_duration` of the Origin of Shell RMRK NFT and reduce it by
	/// `reduce_time_by` This will be executed by the admin account
	fn update_incubation_time(
		admin: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		reduce_time_by: u64,
	) -> Result<BlockNumber, DispatchError>;
}
