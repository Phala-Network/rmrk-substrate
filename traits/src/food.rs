use crate::primitives::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// Incubation Food info
//#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct FoodInfo {
	/// Era that an account last fed food to another Origin of Shell.
	pub era: EraId,
	/// A BoundedVec of (CollectionId, NftId)
	pub origin_of_shells_fed: Vec<(CollectionId, NftId)>,
}

impl FoodInfo {
	pub fn new(era: EraId) -> Self {
		Self { era, origin_of_shells_fed: Vec::new() }
	}
}
