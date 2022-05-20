use crate::primitives::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Incubation Food info
#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct FoodInfo<BoundedOriginOfShellsFed> {
	/// Era that an account last fed food to another Origin of Shell.
	pub era: EraId,
	/// A BoundedVec of (CollectionId, NftId)
	pub origin_of_shells_fed: BoundedOriginOfShellsFed,
}
