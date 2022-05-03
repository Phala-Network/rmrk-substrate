#![cfg_attr(not(feature = "std"), no_std)]

pub mod base;
pub mod career;
pub mod collection;
pub mod nft;
pub mod nft_sale;
pub mod origin_of_shell;
pub mod part;
pub mod preorders;
pub mod priority;
pub mod property;
pub mod race;
pub mod resource;
pub mod spirit;
pub mod status_type;
pub mod theme;
pub mod whitelist;

pub use base::{Base, BaseInfo};
pub use part::{EquippableList, FixedPart, PartType, SlotPart};
pub use theme::{Theme, ThemeProperty};
// pub use part::{PartInfo};
pub use collection::{Collection, CollectionInfo};
pub use nft::{AccountIdOrCollectionNftTuple, Nft, NftInfo};
pub use nft_sale::{NftSaleInfo, NftSaleMetadata};
pub use origin_of_shell::OriginOfShell;
pub use preorders::PreorderInfo;
pub use priority::Priority;
pub use property::Property;
pub use resource::{Resource, ResourceInfo};
pub use spirit::ClaimSpiritTicket;
pub use whitelist::WhitelistClaim;

pub mod primitives {
	pub type CollectionId = u32;
	pub type ResourceId = u32;
	pub type NftId = u32;
	pub type BaseId = u32;
	pub type SlotId = u32;
	pub type PartId = u32;
	pub type ZIndex = u32;
	pub type SerialId = u32;
	pub type PreorderId = u32;
	pub type EraId = u128;
}
