use crate::primitives::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};

/// Incubation Food info
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct FoodInfo {
	/// Era that an account last fed food to another Origin of Shell.
	pub era: EraId,
	/// Food left to feed to another Origin of Shell or to self. Each account gets 5 food to feed
	/// per day.
	pub food_count: u8,
	/// Food left to feed to self. Max of 2 to feed self per day.
	pub food_count_for_self: u8,
}
