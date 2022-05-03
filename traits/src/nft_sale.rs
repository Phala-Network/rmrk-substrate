use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::cmp::Eq;
use sp_core::sr25519;

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

/// Metadata to ensure metadata passed in is signed by the Overlord account
#[derive(Encode, Decode, Clone, Debug, PartialEq, TypeInfo)]
pub struct NftSaleMetadata<BoundedString> {
	pub metadata: BoundedString,
	pub signature: sr25519::Signature,
}