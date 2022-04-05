use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::cmp::Eq;

use crate::{
	career::CareerType, origin_of_shell::OriginOfShellType, race::RaceType, status_type::StatusType,
};

/// NftSaleInfo is used as the value in the StorageDoubleMap that takes key1 as the
/// OriginOfShellType and key2 as the RaceType
#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct NftSaleInfo {
	/// Number of Race Type count
	pub race_count: u32,
	/// Number of Careers within the Race Type count
	pub career_count: u32,
	/// Number of races left to sell
	pub race_for_sale_count: u32,
	/// Number of giveaway races left
	pub race_giveaway_count: u32,
	/// Number of reserved races left
	pub race_reserved_count: u32,
}

// pub trait NftSale<AccountId> {
// 	fn buy_rare_origin_of_shell(
// 		sender: AccountId,
// 		origin_of_shell_type: OriginOfShellType,
// 		race: RaceType,
// 		career: CareerType,
// 	) -> DispatchResult;
// 	fn buy_origin_of_shell_whitelist(
// 		sender: AccountId,
// 		race: RaceType,
// 		career: CareerType,
// 	) -> DispatchResult;
// }
