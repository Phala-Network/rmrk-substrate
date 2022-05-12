use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::cmp::Eq;

/// NftSaleInfo is used as the value in the StorageDoubleMap that takes key1 as the
/// OriginOfShellType and key2 as the RaceType
#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct NftSaleInfo {
	/// Number of Race Type count
	pub race_count: u32,
	/// Number of races left to sell
	pub race_for_sale_count: u32,
	/// Number of giveaway races left
	pub race_giveaway_count: u32,
	/// Number of reserved races left
	pub race_reserved_count: u32,
}