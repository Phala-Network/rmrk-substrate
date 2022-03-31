use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::cmp::Eq;
use frame_support::pallet_prelude::*;

use crate::{career::CareerType, primitives::*, race::RaceType};
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

/// Origin of Shell Types of Hero, Magic & Legendary
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum OriginOfShellType {
	Hero,
	Magic,
	Legendary,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OriginOfShellInfo {
	/// Origin of Shell type of the Origin of Shell RMRK NFT
	pub origin_of_shell_type: OriginOfShellType,
	/// Race type of the Origin of Shell RMRK NFT
	pub race: RaceType,
	/// Career type of the Origin of Shell RMRK NFT
	pub career: CareerType,
	/// Block number when the Origin of Shell started incubation process
	pub start_incubation: u64,
	/// Time duration from `start_incubation` to when the Origin of Shell is ready to incubate
	/// 0 if the Origin of Shell has not started the incubation process
	pub incubation_duration: u64,
}

pub trait OriginOfShell<AccountId, CollectionId, NftId, BlockNumber> {
	/// When a user initiates the incubation process, this function will set the start time for the
	/// incubation process.
	fn set_start_incubation_time(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
	) -> Result<BlockNumber, DispatchError>;
	/// Get the `incubation_duration` of the Origin of Shell RMRK NFT and reduce it by `reduce_time_by`
	/// This will be executed by the admin account
	fn update_incubation_time(
		admin: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		reduce_time_by: u64,
	) -> Result<BlockNumber, DispatchError>;
}
