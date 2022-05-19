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

impl OriginOfShellType {
	pub fn from_u8(num: u8) -> Option<OriginOfShellType> {
		match num {
			0u8 => Some(OriginOfShellType::Prime),
			1u8 => Some(OriginOfShellType::Magic),
			2u8 => Some(OriginOfShellType::Legendary),
			_ => None,
		}
	}
}
