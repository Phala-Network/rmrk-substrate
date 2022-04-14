#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	ensure,
	traits::{Currency, UnixTime},
	transactional, BoundedVec,
};
use frame_system::{ensure_signed, Origin};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::{sr25519, H256};
use sp_io::crypto::sr25519_verify;
use sp_runtime::{
	traits::{One, StaticLookup},
	DispatchResult,
};
use sp_std::prelude::*;

pub use pallet_rmrk_core::types::*;
pub use pallet_rmrk_market;

use rmrk_traits::{
	career::CareerType, origin_of_shell::OriginOfShellType, preorders::PreorderStatus,
	primitives::*, race::RaceType, status_type::StatusType, NftSaleInfo, OriginOfShellInfo,
	PreorderInfo,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

// #[cfg(feature = "std")]
// use serde::{Deserialize, Serialize};
//
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize, PartialEq, Eq))]
// #[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
// pub struct OverlordInfo<AccountId> {
// 	pub admin: AccountId,
// 	pub collection_id: u32,
// }

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		sp_runtime::traits::Zero,
		traits::{
			tokens::nonfungibles::InspectEnumerable, ExistenceRequirement, ReservableCurrency,
		},
	};
	use frame_system::{pallet_prelude::*, Origin};

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type PreorderInfoOf<T> = PreorderInfo<
		<T as frame_system::Config>::AccountId,
		BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
	>;
	//type OverlordInfoOf<T> = OverlordInfo<<T as frame_system::Config>::AccountId>;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The origin which may forcibly buy, sell, list/unlist, offer & withdraw offer on Tokens
		type OverlordOrigin: EnsureOrigin<Self::Origin>;
		/// The market currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// Time in UnixTime
		type Time: UnixTime;
		/// Seconds per Era that will increment the Era storage value every interval
		#[pallet::constant]
		type SecondsPerEra: Get<u64>;
		/// Minimum amount of PHA to claim a Spirit
		#[pallet::constant]
		type MinBalanceToClaimSpirit: Get<BalanceOf<Self>>;
		/// Price of Legendary Origin of Shell Price
		#[pallet::constant]
		type LegendaryOriginOfShellPrice: Get<BalanceOf<Self>>;
		/// Price of Magic Origin of Shell Price
		#[pallet::constant]
		type MagicOriginOfShellPrice: Get<BalanceOf<Self>>;
		/// Price of Hero Origin of Shell Price
		#[pallet::constant]
		type HeroOriginOfShellPrice: Get<BalanceOf<Self>>;
		/// Max mint per Race
		#[pallet::constant]
		type MaxMintPerRace: Get<u32>;
		/// Amount of food per Era
		#[pallet::constant]
		type FoodPerEra: Get<u8>;
		/// Max food an Origin of Shell can be fed per day
		#[pallet::constant]
		type MaxFoodFedPerEra: Get<u16>;
		/// Max food to feed your own Origin of Shell
		#[pallet::constant]
		type MaxFoodFeedSelf: Get<u8>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Stores all of the valid claimed Origin of Shells from the whitelist or preorder
	#[pallet::storage]
	#[pallet::getter(fn claimed_origin_of_shells)]
	pub type ClaimedOriginOfShells<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

	/// Preorder index that is the key to the Preorders StorageMap
	#[pallet::storage]
	#[pallet::getter(fn preorder_index)]
	pub type PreorderIndex<T: Config> = StorageValue<_, PreorderId, ValueQuery>;

	/// Preorder info map for user preorders
	#[pallet::storage]
	#[pallet::getter(fn preorders)]
	pub type Preorders<T: Config> = StorageMap<_, Twox64Concat, PreorderId, PreorderInfoOf<T>>;

	/// Preorder results from the non-whitelist drawing
	#[pallet::storage]
	#[pallet::getter(fn preorder_results)]
	pub type PreorderResults<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		PreorderId,
		PreorderInfoOf<T>,
	>;

	/// Origin of Shells inventory
	#[pallet::storage]
	#[pallet::getter(fn origin_of_shells_inventory)]
	pub type OriginOfShellsInventory<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		OriginOfShellType,
		Blake2_128Concat,
		RaceType,
		NftSaleInfo,
	>;

	/// Stores all the Origin of Shells and the information about the Origin of Shell pertaining to
	/// Hatch times and feeding
	#[pallet::storage]
	#[pallet::getter(fn origin_of_shells)]
	pub type OriginOfShells<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		NftId,
		OriginOfShellInfo,
	>;

	/// Food per Owner where an owner gets 5 food per era
	#[pallet::storage]
	#[pallet::getter(fn get_food_by_owner)]
	pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u8>;

	/// Phala World Zero Day `BlockNumber` this will be used to determine Eras
	#[pallet::storage]
	#[pallet::getter(fn zero_day)]
	pub(super) type ZeroDay<T: Config> = StorageValue<_, u64>;

	/// The current Era from the initial ZeroDay BlockNumber
	#[pallet::storage]
	#[pallet::getter(fn era)]
	pub type Era<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Spirits can be claimed
	#[pallet::storage]
	#[pallet::getter(fn can_claim_spirits)]
	pub type CanClaimSpirits<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Rare Origin of Shells can be purchased
	#[pallet::storage]
	#[pallet::getter(fn can_purchase_rare_origin_of_shells)]
	pub type CanPurchaseRareOriginOfShells<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Origin of Shells can be purchased by whitelist
	#[pallet::storage]
	#[pallet::getter(fn can_purchase_hero_origin_of_shells)]
	pub type CanPurchaseHeroOriginOfShells<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Origin of Shells can be preordered
	#[pallet::storage]
	#[pallet::getter(fn can_preorder_origin_of_shells)]
	pub type CanPreorderOriginOfShells<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Last Day of Sale any Origin of Shell can be purchased
	#[pallet::storage]
	#[pallet::getter(fn last_day_of_sale)]
	pub type LastDayOfSale<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Spirit Collection ID
	#[pallet::storage]
	#[pallet::getter(fn spirit_collection_id)]
	pub type SpiritCollectionId<T: Config> = StorageValue<_, CollectionId, OptionQuery>;

	/// Origin of Shell Collection ID
	#[pallet::storage]
	#[pallet::getter(fn origin_of_shell_collection_id)]
	pub type OriginOfShellCollectionId<T: Config> = StorageValue<_, CollectionId, OptionQuery>;

	/// Career StorageMap count
	#[pallet::storage]
	#[pallet::getter(fn career_type_count)]
	pub type CareerTypeCount<T: Config> = StorageMap<_, Twox64Concat, CareerType, u32, ValueQuery>;

	/// Overlord Admin account of Phala World
	#[pallet::storage]
	#[pallet::getter(fn overlord)]
	pub(super) type Overlord<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: T::BlockNumber) {
			if let Some(zero_day) = <ZeroDay<T>>::get() {
				let current_time = T::Time::now().as_secs();
				if current_time > zero_day {
					let secs_since_zero_day = current_time - zero_day;
					let current_era = <Era<T>>::get();
					if secs_since_zero_day / T::SecondsPerEra::get() > current_era {
						let new_era = Era::<T>::mutate(|era| {
							*era += 1;
							*era
						});
						Self::deposit_event(Event::NewEra { time: current_time, era: new_era });
					}
				}
			}
		}
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// `BlockNumber` of Phala World Zero Day
		pub zero_day: Option<u64>,
		/// Overlord Admin account of Phala World
		pub overlord: Option<T::AccountId>,
		/// Current Era of Phala World
		pub era: u64,
		/// bool for if a Spirit is claimable
		pub can_claim_spirits: bool,
		/// bool for if a Rare Origin of Shell can be purchased
		pub can_purchase_rare_origin_of_shells: bool,
		/// bool for Hero Origin of Shell purchases through whitelist
		pub can_purchase_hero_origin_of_shells: bool,
		/// bool for if an Origin of Shell can be preordered
		pub can_preorder_origin_of_shells: bool,
		/// bool for the last day of sale for Origin of Shell
		pub last_day_of_sale: bool,
		/// CollectionId of Spirit Collection
		pub spirit_collection_id: Option<CollectionId>,
		/// CollectionId of Origin of Shell Collection
		pub origin_of_shell_collection_id: Option<CollectionId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				zero_day: None,
				overlord: None,
				era: 0,
				can_claim_spirits: false,
				can_purchase_rare_origin_of_shells: false,
				can_purchase_hero_origin_of_shells: false,
				can_preorder_origin_of_shells: false,
				last_day_of_sale: false,
				spirit_collection_id: None,
				origin_of_shell_collection_id: None,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T>
	where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
	{
		fn build(&self) {
			if let Some(ref zero_day) = self.zero_day {
				<ZeroDay<T>>::put(zero_day);
			}
			if let Some(ref overlord) = self.overlord {
				<Overlord<T>>::put(overlord);
			}
			let era = self.era;
			<Era<T>>::put(era);
			let can_claim_spirits = self.can_claim_spirits;
			<CanClaimSpirits<T>>::put(can_claim_spirits);
			let can_purchase_rare_origin_of_shells = self.can_purchase_rare_origin_of_shells;
			<CanPurchaseRareOriginOfShells<T>>::put(can_purchase_rare_origin_of_shells);
			let can_purchase_hero_origin_of_shells = self.can_purchase_hero_origin_of_shells;
			<CanPurchaseHeroOriginOfShells<T>>::put(can_purchase_hero_origin_of_shells);
			let can_preorder_origin_of_shells = self.can_preorder_origin_of_shells;
			<CanPreorderOriginOfShells<T>>::put(can_preorder_origin_of_shells);
			let last_day_of_sale = self.last_day_of_sale;
			<LastDayOfSale<T>>::put(last_day_of_sale);
			if let Some(spirit_collection_id) = self.spirit_collection_id {
				<SpiritCollectionId<T>>::put(spirit_collection_id);
			}
			if let Some(origin_of_shell_collection_id) = self.origin_of_shell_collection_id {
				<OriginOfShellCollectionId<T>>::put(origin_of_shell_collection_id);
			}
			// Set initial config for OriginOfShellsInventory
			self::Pallet::<T>::set_initial_origin_of_shell_inventory();
		}
	}

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Phala World clock zero day started
		WorldClockStarted {
			start_time: u64,
		},
		/// Start of a new era
		NewEra {
			time: u64,
			era: u64,
		},
		/// Spirit has been claimed
		SpiritClaimed {
			owner: T::AccountId,
		},
		/// Rare Origin of Shell has been purchased
		RareOriginOfShellPurchased {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// Hero Origin of Shell has been purchased
		HeroOriginOfShellPurchased {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// A chance to get an Origin of Shell through preorder
		OriginOfShellPreordered {
			owner: T::AccountId,
			preorder_id: PreorderId,
		},
		/// Origin of Shell minted from the preorder
		OriginOfShellMinted {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// Spirit collection id was set
		SpiritCollectionIdSet {
			collection_id: CollectionId,
		},
		/// Origin of Shell collection id was set
		OriginOfShellCollectionIdSet {
			collection_id: CollectionId,
		},
		/// Preorder result revealed
		PreorderResultChanged {
			preorder_id: PreorderId,
			status: PreorderStatus,
		},
		/// Refund has been claimed
		RefundWasClaimed {
			preorder_id: PreorderId,
			amount: BalanceOf<T>,
		},
		/// Origin of Shell received food from an account
		OriginOfShellFoodReceived {
			collection_id: CollectionId,
			nft_id: NftId,
			sender: T::AccountId,
			owner: T::AccountId,
		},
		/// Origin of Shell owner has initiated the incubation sequence
		StartedIncubation {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// A top 10 fed origin_of_shell of the era has updated their incubation time
		HatchTimeUpdated {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
			incubation_time: T::BlockNumber,
		},
		/// An origin_of_shell has been awakened
		OriginOfShellHatched {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// Shell has been awakened from an origin_of_shell being hatched and burned
		ShellAwakened {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
			career: u8,
			race: u8,
		},
		/// Origin of Shell inventory updated
		OriginOfShellInventoryUpdated {
			origin_of_shell_type: OriginOfShellType,
		},
		/// Origin of Shell incubation has been disabled & no other origin_of_shells can be hatched
		OriginOfShellIncubationDisabled {
			collection_id: CollectionId,
			can_awaken: bool,
		},
		/// Spirit Claims status has changed
		ClaimSpiritStatusChanged {
			status: bool,
		},
		/// Purchase Rare Origin of Shells status has changed
		PurchaseRareOriginOfShellsStatusChanged {
			status: bool,
		},
		/// Purchase Hero Origin of Shells status changed
		PurchaseHeroOriginOfShellsStatusChanged {
			status: bool,
		},
		/// Preorder Origin of Shells status has changed
		PreorderOriginOfShellsStatusChanged {
			status: bool,
		},
		/// Last Day of Sale status has changed
		LastDayOfSaleStatusChanged {
			status: bool,
		},
		OverlordChanged {
			old_overlord: Option<T::AccountId>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		WorldClockAlreadySet,
		SpiritClaimNotAvailable,
		RareOriginOfShellPurchaseNotAvailable,
		HeroOriginOfShellPurchaseNotAvailable,
		PreorderOriginOfShellNotAvailable,
		SpiritAlreadyClaimed,
		MustOwnSpiritToPurchase,
		OriginOfShellAlreadyPurchased,
		BelowMinimumBalanceThreshold,
		WhitelistVerificationFailed,
		InvalidPurchase,
		NoAvailablePreorderId,
		PreorderClaimNotDetected,
		RefundClaimNotDetected,
		PreorderIsPending,
		NotPreorderOwner,
		RaceMintMaxReached,
		CannotHatchOriginOfShell,
		CannotSendFoodToOriginOfShell,
		NoFoodAvailable,
		OverlordNotSet,
		RequireOverlordAccount,
		InvalidStatusType,
		WrongOriginOfShellType,
		SpiritCollectionNotSet,
		SpiritCollectionIdAlreadySet,
		OriginOfShellCollectionNotSet,
		OriginOfShellCollectionIdAlreadySet,
		OriginOfShellInventoryCorrupted,
		UnableToAddAttributes,
		KeyTooLong,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
	{
		/// Claim a spirit for any account with at least 10 PHA in their account.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic.
		/// - metadata: The metadata of the account that is claiming the spirit.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn claim_spirit(
			origin: OriginFor<T>,
			_mcp_id: u32, // Is this needed?
			_signature: sr25519::Signature,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			ensure!(CanClaimSpirits::<T>::get(), Error::<T>::SpiritClaimNotAvailable);
			let sender = ensure_signed(origin)?;
			let overlord = Overlord::<T>::get().ok_or(Error::<T>::OverlordNotSet)?;
			// Has Spirit Collection been set
			let spirit_collection_id =
				SpiritCollectionId::<T>::get().ok_or(Error::<T>::SpiritCollectionNotSet)?;
			// Check if sender already claimed a spirit
			ensure!(
				pallet_uniques::Pallet::<T>::owned_in_class(&spirit_collection_id, &sender).count() ==
					0,
				Error::<T>::SpiritAlreadyClaimed
			);
			// Check if Balance has minimum required
			ensure!(
				<T as pallet::Config>::Currency::can_reserve(
					&sender,
					T::MinBalanceToClaimSpirit::get()
				),
				Error::<T>::BelowMinimumBalanceThreshold
			);
			// Get NFT ID to be minted
			let spirit_nft_id = pallet_rmrk_core::NextNftId::<T>::get(spirit_collection_id);
			// Mint new Spirit and transfer to sender
			pallet_rmrk_core::Pallet::<T>::mint_nft(
				Origin::<T>::Signed(overlord.clone()).into(),
				sender.clone(),
				spirit_collection_id,
				None,
				None,
				metadata,
			)?;
			// Freeze NFT so it cannot be transferred
			pallet_uniques::Pallet::<T>::freeze(
				Origin::<T>::Signed(overlord).into(),
				spirit_collection_id,
				spirit_nft_id,
			)?;

			Self::deposit_event(Event::SpiritClaimed { owner: sender });

			Ok(())
		}

		// Buy origin of shell of any type during a certain sale interval (i.e. Rare Origin of
		// Shells, Whitelisted Sale and Unlimited Last Day of Sale of any Origin of Shell type.
		// Based on the StatusType passed in with the Origin of Shell Type, Race & Career, will
		// determine if the Origin of Shell can be minted.
		//
		// Parameters:
		// - origin: The origin of the extrinsic.
		// - status_type: The status type of which sale to perform the purchase.
		// - origin_of_shell_type: The type of origin_of_shell to be purchased.
		// - race: The race of the origin_of_shell chosen by the user.
		// - career: The career of the origin_of_shell chosen by the user or auto-generated based on
		//   metadata
		// pub fn mint_origin_of_shell(
		// 	origin: OriginFor<T>,
		// 	status_type: StatusType,
		// 	origin_of_shell_type: OriginOfShellType,
		// 	race: RaceType,
		// 	career: CareerType,
		// ) -> DispatchResult {
		// 	// TODO: Refactor purchases functions
		// 	let sender = ensure_signed(origin.clone())?;
		//
		// 	Ok(())
		// }

		/// Buy a rare origin_of_shell of either type Magic or Legendary. Both Origin of Shell types
		/// will have a set price. These will also be limited in quantity and on a first come, first
		/// serve basis.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic.
		/// - origin_of_shell_type: The type of origin_of_shell to be purchased.
		/// - race: The race of the origin_of_shell chosen by the user.
		/// - career: The career of the origin_of_shell chosen by the user or auto-generated based
		///   on metadata
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn buy_rare_origin_of_shell(
			origin: OriginFor<T>,
			origin_of_shell_type: OriginOfShellType,
			race: RaceType,
			career: CareerType,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			ensure!(
				CanPurchaseRareOriginOfShells::<T>::get() || LastDayOfSale::<T>::get(),
				Error::<T>::RareOriginOfShellPurchaseNotAvailable
			);
			let sender = ensure_signed(origin.clone())?;
			let overlord = Overlord::<T>::get().ok_or(Error::<T>::OverlordNotSet)?;
			// Has Spirit Collection been set
			let spirit_collection_id =
				SpiritCollectionId::<T>::get().ok_or(Error::<T>::SpiritCollectionNotSet)?;
			// Ensure origin_of_shell collection is set
			let origin_of_shell_collection_id = OriginOfShellCollectionId::<T>::get()
				.ok_or(Error::<T>::OriginOfShellCollectionNotSet)?;
			// Must have a spirit to purchase a Origin of Shell
			ensure!(
				pallet_uniques::Pallet::<T>::owned_in_class(&spirit_collection_id, &sender).count() >
					0,
				Error::<T>::MustOwnSpiritToPurchase
			);
			// If not last day of sale, only allowed to purchase one Origin of Shell
			ensure!(
				(!LastDayOfSale::<T>::get() &&
					pallet_uniques::Pallet::<T>::owned_in_class(
						&origin_of_shell_collection_id,
						&sender
					)
					.count() == 0) || LastDayOfSale::<T>::get(),
				Error::<T>::OriginOfShellAlreadyPurchased
			);
			// Get Origin of Shell Price based on Origin of ShellType
			let origin_of_shell_price = match origin_of_shell_type {
				OriginOfShellType::Legendary => T::LegendaryOriginOfShellPrice::get(),
				OriginOfShellType::Magic => T::MagicOriginOfShellPrice::get(),
				_ => return Err(Error::<T>::InvalidPurchase.into()),
			};
			let nft_id = pallet_rmrk_core::NextNftId::<T>::get(origin_of_shell_collection_id);
			// Check if race and career types have mints left
			Self::has_race_type_left(&origin_of_shell_type, &race)?;

			// TODO: Update this for incubation info Define OriginOfShellInfo for storage
			let origin_of_shell = OriginOfShellInfo {
				origin_of_shell_type: origin_of_shell_type.clone(),
				race: race.clone(),
				career: career.clone(),
				start_incubation: 0,
				incubation_duration: 0,
			};

			// Transfer the amount for the rare Origin of Shell NFT then mint the origin_of_shell
			<T as pallet::Config>::Currency::transfer(
				&sender,
				&overlord,
				origin_of_shell_price,
				ExistenceRequirement::KeepAlive,
			)?;
			// Mint Origin of Shell and transfer Origin of Shell to new owner
			pallet_rmrk_core::Pallet::<T>::mint_nft(
				Origin::<T>::Signed(overlord).into(),
				sender.clone(),
				origin_of_shell_collection_id,
				None,
				None,
				metadata,
			)?;
			// Set Origin of Shell Type, Race and Career attributes for NFT
			Self::set_race_and_career_attributes(
				origin_of_shell_collection_id,
				nft_id,
				race.clone(),
				career.clone(),
			)?;

			// Update storage
			Self::decrement_race_type_left(origin_of_shell_type.clone(), race.clone())?;
			Self::increment_race_type(origin_of_shell_type, race)?;
			Self::increment_career_type(career)?;
			OriginOfShells::<T>::insert(origin_of_shell_collection_id, nft_id, origin_of_shell);

			Self::deposit_event(Event::RareOriginOfShellPurchased {
				collection_id: origin_of_shell_collection_id,
				nft_id,
				owner: sender,
			});

			Ok(())
		}

		/// Accounts that have been whitelisted can purchase an Origin of Shell. The only Origin of
		/// Shell type available for this purchase are Hero
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic purchasing the Hero Origin of Shell
		/// - mcp_id: MCP id from the DID protocol linked to the PHA account
		/// - signature: The signature of the account that is claiming the spirit.
		/// - race: The race that the user has chosen (limited # of races)
		/// - career: The career that the user has chosen (unlimited careers)
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn buy_hero_origin_of_shell(
			origin: OriginFor<T>,
			_mcp_id: u32, // Is this needed?
			signature: sr25519::Signature,
			race: RaceType,
			career: CareerType,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			ensure!(
				CanPurchaseHeroOriginOfShells::<T>::get() || LastDayOfSale::<T>::get(),
				Error::<T>::HeroOriginOfShellPurchaseNotAvailable
			);
			let sender = ensure_signed(origin.clone())?;
			let overlord = Overlord::<T>::get().ok_or(Error::<T>::OverlordNotSet)?;
			// Check if valid whitelist account
			ensure!(
				Self::verify_claim(sender.clone(), metadata.clone(), signature),
				Error::<T>::WhitelistVerificationFailed
			);
			// Has Spirit Collection been set
			let spirit_collection_id =
				SpiritCollectionId::<T>::get().ok_or(Error::<T>::SpiritCollectionNotSet)?;
			// Ensure origin_of_shell collection is set
			let origin_of_shell_collection_id = OriginOfShellCollectionId::<T>::get()
				.ok_or(Error::<T>::OriginOfShellCollectionNotSet)?;
			// Must have a spirit to purchase a Origin of Shell
			ensure!(
				pallet_uniques::Pallet::<T>::owned_in_class(&spirit_collection_id, &sender).count() >
					0,
				Error::<T>::MustOwnSpiritToPurchase
			);
			// If not last day of sale, only allowed to purchase one Origin of Shell
			ensure!(
				(!LastDayOfSale::<T>::get() &&
					pallet_uniques::Pallet::<T>::owned_in_class(
						&origin_of_shell_collection_id,
						&sender
					)
					.count() == 0) || LastDayOfSale::<T>::get(),
				Error::<T>::OriginOfShellAlreadyPurchased
			);
			let nft_id = pallet_rmrk_core::NextNftId::<T>::get(origin_of_shell_collection_id);
			// Get Hero Origin of Shell price
			let origin_of_shell_price = T::HeroOriginOfShellPrice::get();
			// Check if race and career types have mints left
			Self::has_race_type_left(&OriginOfShellType::Hero, &race)?;

			// Define OriginOfShellInfo for storage
			let origin_of_shell = OriginOfShellInfo {
				origin_of_shell_type: OriginOfShellType::Hero,
				race: race.clone(),
				career: career.clone(),
				start_incubation: 0,
				incubation_duration: 0,
			};

			// Transfer the amount for the rare Origin of Shell NFT then mint the origin_of_shell
			<T as pallet::Config>::Currency::transfer(
				&sender,
				&overlord,
				origin_of_shell_price,
				ExistenceRequirement::KeepAlive,
			)?;
			// Mint Origin of Shell and transfer Origin of Shell to new owner
			pallet_rmrk_core::Pallet::<T>::mint_nft(
				Origin::<T>::Signed(overlord).into(),
				sender.clone(),
				origin_of_shell_collection_id,
				None,
				None,
				metadata,
			)?;
			// Set Origin of Shell Type, Race and Career attributes for NFT
			Self::set_race_and_career_attributes(
				origin_of_shell_collection_id,
				nft_id,
				race.clone(),
				career.clone(),
			)?;

			// Update storage
			Self::decrement_race_type_left(OriginOfShellType::Hero, race.clone())?;
			Self::increment_race_type(OriginOfShellType::Hero, race)?;
			Self::increment_career_type(career)?;
			OriginOfShells::<T>::insert(origin_of_shell_collection_id, nft_id, origin_of_shell);

			Self::deposit_event(Event::HeroOriginOfShellPurchased {
				collection_id: origin_of_shell_collection_id,
				nft_id,
				owner: sender,
			});

			Ok(())
		}

		/// Users can pre-order an Origin of Shell. This will enable users that are non-whitelisted
		/// to be added to the queue of users that can claim Origin of Shells. Those that come after
		/// the whitelist pre-sale will be able to win the chance to acquire an Origin of Shell
		/// based on their choice of race and career as they will have a limited quantity.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic preordering the origin_of_shell
		/// - race: The race that the user has chosen (limited # of races)
		/// - career: The career that the user has chosen (limited # of careers)
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn preorder_origin_of_shell(
			origin: OriginFor<T>,
			race: RaceType,
			career: CareerType,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			ensure!(
				CanPreorderOriginOfShells::<T>::get(),
				Error::<T>::PreorderOriginOfShellNotAvailable
			);
			let sender = ensure_signed(origin)?;
			// Has Spirit Collection been set
			let spirit_collection_id =
				SpiritCollectionId::<T>::get().ok_or(Error::<T>::SpiritCollectionNotSet)?;
			// Must have a spirit to purchase a Origin of Shell
			ensure!(
				pallet_uniques::Pallet::<T>::owned_in_class(&spirit_collection_id, &sender).count() >
					0,
				Error::<T>::MustOwnSpiritToPurchase
			);

			// Get preorder_id for new preorder
			let preorder_id =
				<PreorderIndex<T>>::try_mutate(|n| -> Result<PreorderId, DispatchError> {
					let id = *n;
					ensure!(id != PreorderId::max_value(), Error::<T>::NoAvailablePreorderId);
					*n += 1;
					Ok(id)
				})?;

			let preorder = PreorderInfo {
				owner: sender.clone(),
				race: race.clone(),
				career: career.clone(),
				metadata,
				preorder_status: PreorderStatus::Pending,
			};
			// Reserve currency for the preorder at the Hero origin_of_shell price
			<T as pallet::Config>::Currency::reserve(&sender, T::HeroOriginOfShellPrice::get())?;

			Preorders::<T>::insert(preorder_id, preorder);

			Self::deposit_event(Event::OriginOfShellPreordered { owner: sender, preorder_id });

			Ok(())
		}

		/// This is an admin only function that will be used to set a preorder's status from Pending
		/// to either Chosen or NotChosen
		///
		/// Parameters:
		/// `origin`: Expected to come from Overlord admin account
		/// `preorder_id`: Preorder id to change preorder status
		/// `status`: Either Chosen or NotChosen
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_preorder_status(
			origin: OriginFor<T>,
			preorder_id: PreorderId,
			status: PreorderStatus,
		) -> DispatchResult {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			Self::ensure_overlord(sender)?;
			// Ensure value exists
			ensure!(Preorders::<T>::contains_key(preorder_id), Error::<T>::NoAvailablePreorderId);
			// Change status of preorder and add to PreorderResult
			let mut preorder_info =
				Preorders::<T>::take(preorder_id).ok_or(Error::<T>::NoAvailablePreorderId)?;
			preorder_info.preorder_status = status.clone();
			PreorderResults::<T>::insert(&preorder_info.owner, preorder_id, &preorder_info);

			Self::deposit_event(Event::PreorderResultChanged { preorder_id, status });

			Ok(())
		}

		/// Claim function for bulk minting chosen preorders for the sender
		///
		/// Parameters:
		/// `origin`: Sender
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn claim_chosen_preorders(origin: OriginFor<T>) -> DispatchResult {
			ensure!(
				!CanPreorderOriginOfShells::<T>::get(),
				Error::<T>::PreorderOriginOfShellNotAvailable
			);
			let sender = ensure_signed(origin)?;
			let overlord = Overlord::<T>::get().ok_or(Error::<T>::OverlordNotSet)?;

			// Has Spirit Collection been set
			let spirit_collection_id =
				SpiritCollectionId::<T>::get().ok_or(Error::<T>::SpiritCollectionNotSet)?;
			// Must have a spirit to purchase a Origin of Shell
			ensure!(
				pallet_uniques::Pallet::<T>::owned_in_class(&spirit_collection_id, &sender).count() >
					0,
				Error::<T>::MustOwnSpiritToPurchase
			);
			// Must have origin of shell collection created
			let origin_of_shell_collection_id = OriginOfShellCollectionId::<T>::get()
				.ok_or(Error::<T>::OriginOfShellCollectionNotSet)?;
			ensure!(
				(pallet_uniques::Pallet::<T>::owned_in_class(
					&origin_of_shell_collection_id,
					&sender
				)
				.count() == 0) || LastDayOfSale::<T>::get(),
				Error::<T>::OriginOfShellAlreadyPurchased
			);
			// Get price of hero origin of shell
			let hero_origin_of_shell_price = T::HeroOriginOfShellPrice::get();
			// Check if any preorders were chosen
			let preorders_iter = PreorderResults::<T>::drain_prefix(sender.clone());
			for (preorder_id, preorder) in preorders_iter {
				let preorder_status = preorder.clone().preorder_status;
				match preorder_status {
					PreorderStatus::Chosen => {
						// TODO refactor Define OriginOfShellInfo for storage
						let origin_of_shell = OriginOfShellInfo {
							origin_of_shell_type: OriginOfShellType::Hero,
							race: preorder.race.clone(),
							career: preorder.career.clone(),
							start_incubation: 0,
							incubation_duration: 0,
						};
						// Next NFT ID of Collection
						let nft_id =
							pallet_rmrk_core::NextNftId::<T>::get(origin_of_shell_collection_id);
						// Get payment from owner's reserve
						<T as pallet::Config>::Currency::unreserve(
							&sender,
							hero_origin_of_shell_price,
						);
						<T as pallet::Config>::Currency::transfer(
							&sender,
							&overlord,
							hero_origin_of_shell_price,
							ExistenceRequirement::KeepAlive,
						)?;
						// Mint Origin of Shell and transfer Origin of Shell to new owner
						pallet_rmrk_core::Pallet::<T>::mint_nft(
							Origin::<T>::Signed(overlord.clone()).into(),
							preorder.owner.clone(),
							origin_of_shell_collection_id,
							None,
							None,
							preorder.metadata,
						)?;

						OriginOfShells::<T>::insert(
							origin_of_shell_collection_id,
							nft_id,
							origin_of_shell,
						);

						Self::deposit_event(Event::OriginOfShellMinted {
							collection_id: origin_of_shell_collection_id,
							nft_id,
							owner: preorder.owner,
						});
					},
					PreorderStatus::NotChosen => {
						// Re-insert into Storage as this will be handled in the claim refund
						// preorders function
						PreorderResults::<T>::insert(sender.clone(), preorder_id, preorder);
					},
					PreorderStatus::Pending => return Err(Error::<T>::PreorderIsPending.into()),
				}
			}
			Ok(())
		}

		/// Claim function for refunds for not chosen preorders for the sender
		///
		/// Parameters:
		/// `origin`: Sender
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn claim_refund_preorders(origin: OriginFor<T>) -> DispatchResult {
			ensure!(
				!CanPreorderOriginOfShells::<T>::get(),
				Error::<T>::PreorderOriginOfShellNotAvailable
			);
			let sender = ensure_signed(origin)?;
			// Get the amount cost for Preorder
			let hero_origin_of_shell_price = T::HeroOriginOfShellPrice::get();
			// Check if any preorders were chosen
			let preorders_iter = PreorderResults::<T>::drain_prefix(sender.clone());
			for (preorder_id, preorder) in preorders_iter {
				let preorder_status = preorder.clone().preorder_status;
				match preorder_status {
					PreorderStatus::NotChosen => {
						ensure!(
							sender.clone() == preorder.owner.clone(),
							Error::<T>::NotPreorderOwner
						);
						// Get payment from owner's reserve
						<T as pallet::Config>::Currency::unreserve(
							&sender,
							hero_origin_of_shell_price,
						);

						Self::deposit_event(Event::RefundWasClaimed {
							preorder_id,
							amount: hero_origin_of_shell_price,
						});
					},
					PreorderStatus::Chosen => {
						// Re-insert into Storage as this will be handled in the claim Chosen
						// preorders function
						PreorderResults::<T>::insert(sender.clone(), preorder_id, preorder);
					},
					PreorderStatus::Pending => return Err(Error::<T>::PreorderIsPending.into()),
				}
			}
			Ok(())
		}

		/// Once users have received their origin_of_shells and the start incubation event has been
		/// triggered, they can start the incubation process and a timer will start for the
		/// origin_of_shell to awaken at a designated time. Origin of Shells can reduce their time
		/// by being in the top 10 of origin_of_shell's fed per era.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic starting the incubation process
		/// - collection_id: The collection id of the Origin of Shell RMRK NFT
		/// - nft_id: The NFT id of the Origin of Shell RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn start_incubation(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// Feed another origin_of_shell to the current origin_of_shell being incubated. This will
		/// reduce the time left to incubation if the origin_of_shell is in the top 10 of
		/// origin_of_shells fed that era.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic feeding the origin_of_shell
		/// - collection_id: The collection id of the Origin of Shell RMRK NFT
		/// - nft_id: The NFT id of the Origin of Shell RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn feed_origin_of_shell(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// Hatch the origin_of_shell that is currently being hatched. This will trigger the end of
		/// the incubation process and the origin_of_shell will be burned. After burning, the user
		/// will receive the awakened Shell RMRK NFT
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic incubation the origin_of_shell
		/// - collection_id: The collection id of the Origin of Shell RMRK NFT
		/// - nft_id: The NFT id of the Origin of Shell RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn hatch_origin_of_shell(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// This is an admin function to update origin_of_shells incubation times based on being in
		/// the top 10 of fed origin_of_shells within that era
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic updating the origin_of_shells incubation times
		/// - collection_id: The collection id of the Origin of Shell RMRK NFT
		/// - nft_id: The NFT id of the Origin of Shell RMRK NFT
		/// - reduced_time: The amount of time the origin_of_shell will be reduced by
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn update_incubation_time(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			reduced_time: u64,
		) -> DispatchResult {
			// Ensure OverlordOrigin makes call
			let sender = ensure_signed(origin)?;
			Self::ensure_overlord(sender)?;

			Ok(())
		}

		/// Privileged function set the Overlord Admin account of Phala World
		///
		/// Parameters:
		/// - origin: Expected to be called by `OverlordOrigin`
		/// - new_overlord: T::AccountId
		#[pallet::weight(0)]
		pub fn set_overlord(
			origin: OriginFor<T>,
			new_overlord: T::AccountId,
		) -> DispatchResultWithPostInfo {
			// This is a public call, so we ensure that the origin is some signed account.
			ensure_root(origin)?;
			let old_overlord = <Overlord<T>>::get();

			Overlord::<T>::put(&new_overlord);
			Self::deposit_event(Event::OverlordChanged { old_overlord });
			// GameOverlord user does not pay a fee
			Ok(Pays::No.into())
		}

		/// Phala World Zero Day is set to begin the tracking of the official time starting at the
		/// current timestamp when `initialize_world_clock` is called by the `Overlord`
		///
		/// Parameters:
		/// `origin`: Expected to be called by `Overlord` admin account
		#[pallet::weight(0)]
		pub fn initialize_world_clock(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			Self::ensure_overlord(sender)?;
			// Ensure ZeroDay is None as this can only be set once
			ensure!(Self::zero_day() == None, Error::<T>::WorldClockAlreadySet);

			let zero_day = T::Time::now().as_secs();

			ZeroDay::<T>::put(zero_day);
			Self::deposit_event(Event::WorldClockStarted { start_time: zero_day });

			Ok(Pays::No.into())
		}

		/// Privileged function to set the status for one of the defined StatusTypes like
		/// ClaimSpirits, PurchaseRareOriginOfShells, or PreorderOriginOfShells to enable
		/// functionality in Phala World
		///
		/// Parameters:
		/// - `origin` - Expected Overlord admin account to set the status
		/// - `status` - `bool` to set the status to
		/// - `status_type` - `StatusType` to set the status for
		#[pallet::weight(0)]
		pub fn set_status_type(
			origin: OriginFor<T>,
			status: bool,
			status_type: StatusType,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			Self::ensure_overlord(sender)?;
			// Match StatusType and call helper function to set status
			match status_type {
				StatusType::ClaimSpirits => Self::set_claim_spirits_status(status)?,
				StatusType::PurchaseRareOriginOfShells =>
					Self::set_purchase_rare_origin_of_shells_status(status)?,
				StatusType::PurchaseHeroOriginOfShells =>
					Self::set_purchase_hero_origin_of_shells_status(status)?,
				StatusType::PreorderOriginOfShells =>
					Self::set_preorder_origin_of_shells_status(status)?,
				StatusType::LastDayOfSale => Self::set_last_day_of_sale_status(status)?,
			}
			Ok(Pays::No.into())
		}

		/// Update for the non-whitelist preorder period amount of races & giveaways available for
		/// the Origin of Shell NFTs. This is a privileged function and can only be executed by the
		/// Overlord account. Update the OriginOfShellInventory counts by incrementing them based on
		/// the defined counts
		///
		/// Parameters:
		/// - `origin` - Expected Overlord admin account
		/// - `origin_of_shell_type` - Type of Origin of Shell
		/// - `for_sale_count` - Number of Origin of Shells for sale
		/// - `giveaway_count` - Number of Origin of Shells for giveaways
		/// - `reserve_count` - Number of Origin of Shells to be reserved
		#[pallet::weight(0)]
		pub fn update_origin_of_shell_type_counts(
			origin: OriginFor<T>,
			origin_of_shell_type: OriginOfShellType,
			for_sale_count: u32,
			giveaway_count: u32,
		) -> DispatchResult {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			Self::ensure_overlord(sender)?;
			// Ensure they are updating the OriginOfShellType::Hero
			ensure!(
				origin_of_shell_type == OriginOfShellType::Hero,
				Error::<T>::WrongOriginOfShellType
			);
			// Mutate the existing storage for the Hero Origin of Shells
			Self::update_nft_sale_info(
				origin_of_shell_type.clone(),
				RaceType::AISpectre,
				for_sale_count,
				giveaway_count,
			)?;
			Self::update_nft_sale_info(
				origin_of_shell_type.clone(),
				RaceType::Cyborg,
				for_sale_count,
				giveaway_count,
			)?;
			Self::update_nft_sale_info(
				origin_of_shell_type.clone(),
				RaceType::Pandroid,
				for_sale_count,
				giveaway_count,
			)?;
			Self::update_nft_sale_info(
				origin_of_shell_type.clone(),
				RaceType::XGene,
				for_sale_count,
				giveaway_count,
			)?;

			Self::deposit_event(Event::OriginOfShellInventoryUpdated { origin_of_shell_type });

			Ok(())
		}

		/// Privileged function to set the collection id for the Spirits collection
		///
		/// Parameters:
		/// - `origin` - Expected Overlord admin account to set the Spirit Collection ID
		/// - `collection_id` - Collection ID of the Spirit Collection
		#[pallet::weight(0)]
		pub fn set_spirit_collection_id(
			origin: OriginFor<T>,
			collection_id: CollectionId,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			Self::ensure_overlord(sender)?;
			// If Spirit Collection ID is greater than 0 then the collection ID was already set
			ensure!(
				SpiritCollectionId::<T>::get().is_none(),
				Error::<T>::SpiritCollectionIdAlreadySet
			);
			<SpiritCollectionId<T>>::put(collection_id);

			Self::deposit_event(Event::SpiritCollectionIdSet { collection_id });

			Ok(Pays::No.into())
		}

		/// Privileged function to set the collection id for the Origin of Shell collection
		///
		/// Parameters:
		/// - `origin` - Expected Overlord admin account to set the Origin of Shell Collection ID
		/// - `collection_id` - Collection ID of the Origin of Shell Collection
		#[pallet::weight(0)]
		pub fn set_origin_of_shell_collection_id(
			origin: OriginFor<T>,
			collection_id: CollectionId,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			Self::ensure_overlord(sender)?;
			// If Origin of Shell Collection ID is greater than 0 then the collection ID was already
			// set
			ensure!(
				OriginOfShellCollectionId::<T>::get().is_none(),
				Error::<T>::OriginOfShellCollectionIdAlreadySet
			);
			<OriginOfShellCollectionId<T>>::put(collection_id);

			Self::deposit_event(Event::OriginOfShellCollectionIdSet { collection_id });

			Ok(Pays::No.into())
		}
	}
}

impl<T: Config> Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	/// Verify the whitelist status of an Account that has purchased origin of shell. Serialize the
	/// evidence with the claimer & metadata then verify against the expected results
	/// by calling sr25519 verify function
	///
	/// Parameters:
	/// - claimer: AccountId of the account in the whitelist
	/// - metadata: Metadata passed in associated with the claimer
	/// - signature: Signature passed in by the claimer
	pub fn verify_claim(
		claimer: T::AccountId,
		metadata: BoundedVec<u8, T::StringLimit>,
		signature: sr25519::Signature,
	) -> bool {
		// Serialize the evidence
		let msg = Encode::encode(&(claimer, metadata));
		if let Some(overlord) = <Overlord<T>>::get() {
			let encode_overlord = T::AccountId::encode(&overlord);
			let h256_overlord = H256::from_slice(&encode_overlord);
			let overlord_key = sr25519::Public::from_h256(h256_overlord);
			// verify claim
			sp_io::crypto::sr25519_verify(&signature, &msg, &overlord_key)
		} else {
			false
		}
	}

	/// Helper function to ensure Overlord account is the sender
	///
	/// Parameters:
	/// - `sender`: Account origin that made the call to check if Overlord account
	fn ensure_overlord(sender: T::AccountId) -> DispatchResult {
		ensure!(
			Self::overlord().map_or(false, |k| sender == k),
			Error::<T>::RequireOverlordAccount
		);
		Ok(())
	}

	/// Set Spirit Claims with the Overlord admin Account to allow users to claim their
	/// Spirits through the `claim_spirits()` function
	///
	/// Parameters:
	/// - `status`: Status to set CanClaimSpirits StorageValue
	fn set_claim_spirits_status(status: bool) -> DispatchResult {
		<CanClaimSpirits<T>>::put(status);

		Self::deposit_event(Event::ClaimSpiritStatusChanged { status });

		Ok(())
	}

	/// Set Rare Origin of Shells status for purchase with the Overlord Admin Account to allow
	/// users to purchase either Legendary or Magic Origin of Shells
	///
	/// Parameters:
	/// `status`: Status to set CanPurchaseRareOriginOfShells StorageValue
	fn set_purchase_rare_origin_of_shells_status(status: bool) -> DispatchResult {
		<CanPurchaseRareOriginOfShells<T>>::put(status);

		Self::deposit_event(Event::PurchaseRareOriginOfShellsStatusChanged { status });

		Ok(())
	}

	/// Set Hero Origin of Shells status for purchase with the Overlord Admin Account to allow
	/// users to purchase Hero Origin of Shells
	///
	/// Parameters:
	/// `status`: Status to set CanPurchaseOriginOfShellsWhitelist StorageValue
	fn set_purchase_hero_origin_of_shells_status(status: bool) -> DispatchResult {
		<CanPurchaseHeroOriginOfShells<T>>::put(status);

		Self::deposit_event(Event::PurchaseHeroOriginOfShellsStatusChanged { status });

		Ok(())
	}

	/// Set status of Preordering origin_of_shells with the Overlord Admin Account to allow
	/// users to preorder origin_of_shells through the `preorder_origin_of_shell()` function
	///
	/// Parameters:
	/// - `status`: Status to set CanPreorderOriginOfShells StorageValue
	fn set_preorder_origin_of_shells_status(status: bool) -> DispatchResult {
		<CanPreorderOriginOfShells<T>>::put(status);

		Self::deposit_event(Event::PreorderOriginOfShellsStatusChanged { status });

		Ok(())
	}

	/// Set status of last day of sale for origin of shells with the Overlord Admin Account to allow
	/// users to purchase any origin of shell
	///
	/// Parameters:
	/// - `status`: Status to set LastDayOfSale StorageValue
	fn set_last_day_of_sale_status(status: bool) -> DispatchResult {
		<LastDayOfSale<T>>::put(status);

		Self::deposit_event(Event::PreorderOriginOfShellsStatusChanged { status });

		Ok(())
	}

	/// Set initial OriginOfShellInventory values in the StorageDoubleMap. Key1 will be of
	/// OriginOfShellType and Key2 will be the RaceType and the Value will be NftSaleInfo struct
	/// containing the information for the NFT sale. Initial config will look as follows:
	/// `<Legendary>,<RaceType> => NftSaleInfo { race_count: 0, career_count: 0,
	/// race_for_sale_count: 1, race_giveaway_count: 0, race_reserved_count: 1 }`
	/// `<Magic>,<RaceType> => NftSaleInfo { race_count: 0, career_count: 0, race_for_sale_count:
	/// 15, race_giveaway_count: 0, race_reserved_count: 5 }`
	/// `<Hero>,<RaceType> => NftSaleInfo { race_count: 0, career_count: 0, race_for_sale_count:
	/// 1250, race_giveaway_count: 50, race_reserved_count: 0 }`
	fn set_initial_origin_of_shell_inventory() {
		// 3 OriginOfShellType Hero, Magic & Legendary and 4 different RaceType Cyborg, AISpectre,
		// XGene & Pandroid
		// TODO Refactor
		let legendary_nft_sale_info = NftSaleInfo {
			race_count: 0,
			race_for_sale_count: 1,
			race_giveaway_count: 0,
			race_reserved_count: 1,
		};
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Legendary,
			RaceType::AISpectre,
			legendary_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Legendary,
			RaceType::Cyborg,
			legendary_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Legendary,
			RaceType::Pandroid,
			legendary_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Legendary,
			RaceType::XGene,
			legendary_nft_sale_info,
		);
		let magic_nft_sale_info = NftSaleInfo {
			race_count: 0,
			race_for_sale_count: 15,
			race_giveaway_count: 0,
			race_reserved_count: 5,
		};
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Magic,
			RaceType::AISpectre,
			magic_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Magic,
			RaceType::Cyborg,
			magic_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Magic,
			RaceType::Pandroid,
			magic_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Magic,
			RaceType::XGene,
			magic_nft_sale_info,
		);
		let hero_nft_sale_info = NftSaleInfo {
			race_count: 0,
			race_for_sale_count: 1250,
			race_giveaway_count: 50,
			race_reserved_count: 0,
		};
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Hero,
			RaceType::AISpectre,
			hero_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Hero,
			RaceType::Cyborg,
			hero_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Hero,
			RaceType::Pandroid,
			hero_nft_sale_info.clone(),
		);
		OriginOfShellsInventory::<T>::insert(
			OriginOfShellType::Hero,
			RaceType::XGene,
			hero_nft_sale_info,
		);
	}

	/// Update the NftSaleInfo for a given OriginOfShellType and RaceType
	///
	/// Parameters:
	/// - `origin_of_shell_type`: OriginOfShellType to update in OriginOfShellInventory
	/// - `race`: RaceType to update in OriginOfShellInventory
	/// - `for_sale_count`: count to increment the for sale count
	/// - `giveaway_count`: count to increment the race giveaway count
	fn update_nft_sale_info(
		origin_of_shell_type: OriginOfShellType,
		race: RaceType,
		for_sale_count: u32,
		giveaway_count: u32,
	) -> DispatchResult {
		OriginOfShellsInventory::<T>::try_mutate_exists(
			origin_of_shell_type,
			race,
			|nft_sale_info| -> DispatchResult {
				if let Some(nft_sale_info) = nft_sale_info {
					nft_sale_info.race_for_sale_count =
						nft_sale_info.race_for_sale_count.saturating_add(for_sale_count);
					nft_sale_info.race_giveaway_count =
						nft_sale_info.race_giveaway_count.saturating_add(giveaway_count);
				}
				Ok(())
			},
		)?;

		Ok(())
	}

	/// Set the race and career attributes for a Origin of Shell NFT
	///
	/// Parameters:
	/// - `collection_id`: Collection id of the Origin of Shell NFT
	/// - `nft_id`: NFT id of the Origin of Shell NFT
	/// - `race`: Race attribute to set for the Origin of Shell NFT
	/// - `career`: Career attribute to set for the Origin of Shell NFT
	fn set_race_and_career_attributes(
		collection_id: CollectionId,
		nft_id: NftId,
		race: RaceType,
		career: CareerType,
	) -> DispatchResult {
		let overlord = Overlord::<T>::get().ok_or(Error::<T>::OverlordNotSet)?;
		let race_key: BoundedVec<u8, T::KeyLimit> = self::Pallet::<T>::to_boundedvec_key("race")?;
		let race_value = race.encode().try_into().expect("[race] should not fail");
		// Set Race
		pallet_uniques::Pallet::<T>::set_attribute(
			Origin::<T>::Signed(overlord.clone()).into(),
			collection_id,
			Some(nft_id),
			race_key,
			race_value,
		)?;
		let career_key = self::Pallet::<T>::to_boundedvec_key("career")?;
		let career_value = career.encode().try_into().expect("[career] should not fail");
		// Set Career
		pallet_uniques::Pallet::<T>::set_attribute(
			Origin::<T>::Signed(overlord).into(),
			collection_id,
			Some(nft_id),
			career_key,
			career_value,
		)?;

		Ok(())
	}

	/// Decrement CareerType count for the `career`
	///
	/// Parameters:
	/// - `career`: The Career to increment count
	fn decrement_career_type(career: CareerType) -> DispatchResult {
		CareerTypeCount::<T>::mutate(career, |career_count| {
			*career_count -= 1;
			*career_count
		});

		Ok(())
	}

	/// Increment RaceType count for the `race`
	///
	/// Parameters:
	/// - `origin_of_shell_type`: Origin of Shell type
	/// - `race`: The Career to increment count
	fn increment_race_type(
		origin_of_shell_type: OriginOfShellType,
		race: RaceType,
	) -> DispatchResult {
		OriginOfShellsInventory::<T>::try_mutate_exists(
			origin_of_shell_type,
			race,
			|nft_sale_info| -> DispatchResult {
				if let Some(nft_sale_info) = nft_sale_info {
					nft_sale_info.race_count += 1;
				}
				Ok(())
			},
		)?;

		Ok(())
	}

	/// Increment CareerType count for the `career`
	///
	/// Parameters:
	/// - `career`: The Career to increment count
	fn increment_career_type(career: CareerType) -> DispatchResult {
		CareerTypeCount::<T>::mutate(career, |career_count| {
			*career_count += 1;
			*career_count
		});

		Ok(())
	}

	/// Decrement RaceType count for the `race`
	///
	/// Parameters:
	/// - `race`: The Race to increment count
	fn decrement_race_type_left(
		origin_of_shell_type: OriginOfShellType,
		race: RaceType,
	) -> DispatchResult {
		OriginOfShellsInventory::<T>::try_mutate_exists(
			origin_of_shell_type,
			race,
			|nft_sale_info| -> DispatchResult {
				if let Some(nft_sale_info) = nft_sale_info {
					nft_sale_info.race_for_sale_count -= 1;
				}
				Ok(())
			},
		)?;

		Ok(())
	}

	/// Verify if the chosen Race has reached the max limit
	///
	/// Parameters:
	/// - `race`: The Race to check
	fn has_race_type_left(
		origin_of_shell_type: &OriginOfShellType,
		race: &RaceType,
	) -> DispatchResult {
		if let Some(nft_sale_info) = OriginOfShellsInventory::<T>::get(origin_of_shell_type, race) {
			ensure!(nft_sale_info.race_for_sale_count > 0, Error::<T>::RaceMintMaxReached);
		} else {
			return Err(Error::<T>::OriginOfShellInventoryCorrupted.into())
		}

		Ok(())
	}

	fn to_boundedvec_key(name: &str) -> Result<BoundedVec<u8, T::KeyLimit>, Error<T>> {
		name.as_bytes().to_vec().try_into().map_err(|_| Error::<T>::KeyTooLong)
	}
}
