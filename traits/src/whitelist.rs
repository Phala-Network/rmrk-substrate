use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::sr25519;

#[derive(Encode, Decode, Clone, Debug, PartialEq, TypeInfo)]
pub struct WhitelistClaim<AccountId, BoundedString> {
	pub account: AccountId,
	pub metadata: BoundedString,
	pub signature: sr25519::Signature,
}
