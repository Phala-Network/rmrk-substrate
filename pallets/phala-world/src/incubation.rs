//! Phala World Incubation Pallet

use frame_support::{
	ensure,
	pallet_prelude::Get,
	traits::{
		tokens::{
			nonfungibles::{Inspect, InspectEnumerable},
			ExistenceRequirement,
		},
		Currency, UnixTime,
	},
	transactional, BoundedVec,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use sp_runtime::DispatchResult;
use sp_std::{vec, vec::Vec};

pub use crate::pallet_pw_nft_sale;
pub use pallet_rmrk_core::types::*;
pub use pallet_rmrk_market;

use rmrk_traits::{
	career::CareerType, food::FoodInfo, origin_of_shell::OriginOfShellType, primitives::*,
	race::RaceType, resource::ResourceInfo, AccountIdOrCollectionNftTuple,
};

pub type ResourceOf<T, R> =
	ResourceInfo<BoundedVec<u8, R>, BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>>;

pub use self::pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::Origin;
	use pallet_rmrk_core::BoundedResource;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_rmrk_core::Config + pallet_pw_nft_sale::Config
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Amount of food per Era
		#[pallet::constant]
		type FoodPerEra: Get<u8>;
		/// Max food to feed your own Origin of Shell
		#[pallet::constant]
		type MaxFoodFeedSelf: Get<u8>;
		/// Duration of incubation process
		#[pallet::constant]
		type IncubationDurationSec: Get<u64>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Info on Origin of Shells that the Owner has fed
	#[pallet::storage]
	#[pallet::getter(fn food_by_owners)]
	pub type FoodByOwners<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, FoodInfo, OptionQuery>;

	/// Total food fed to an Origin of Shell per Era
	#[pallet::storage]
	#[pallet::getter(fn origin_of_shell_food_stats)]
	pub type OriginOfShellFoodStats<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		(CollectionId, NftId),
		Blake2_128Concat,
		EraId,
		u32,
		ValueQuery,
	>;

	/// Official hatch time for all Origin of Shells
	#[pallet::storage]
	#[pallet::getter(fn official_hatch_time)]
	pub type OfficialHatchTime<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Expected hatch Timestamp for an Origin of Shell that started the incubation process
	#[pallet::storage]
	#[pallet::getter(fn hatch_time)]
	pub type HatchTimes<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, CollectionId, Blake2_128Concat, NftId, u64>;

	/// A bool value to determine if accounts can start incubation of Origin of Shells
	#[pallet::storage]
	#[pallet::getter(fn can_start_incubation)]
	pub type CanStartIncubation<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Collection ID of the Shell NFT
	#[pallet::storage]
	#[pallet::getter(fn shell_collection_id)]
	pub type ShellCollectionId<T: Config> = StorageValue<_, CollectionId>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// CanStartIncubation status changed and set official hatch time
		CanStartIncubationStatusChanged { status: bool, start_time: u64, official_hatch_time: u64 },
		/// Origin of Shell received food from an account
		OriginOfShellReceivedFood {
			collection_id: CollectionId,
			nft_id: NftId,
			sender: T::AccountId,
		},
		/// A top 10 fed origin_of_shell of the era has updated their incubation time
		HatchTimeUpdated {
			collection_id: CollectionId,
			nft_id: NftId,
			old_hatch_time: u64,
			new_hatch_time: u64,
		},
		/// Shell Collection ID is set
		ShellCollectionIdSet { collection_id: CollectionId },
		/// Shell has been awakened from an origin_of_shell being hatched and burned
		ShellAwakened { collection_id: CollectionId, nft_id: NftId, owner: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		StartIncubationNotAvailable,
		HatchingInProgress,
		CannotHatchOriginOfShell,
		CannotSendFoodToOriginOfShell,
		MaxFoodFedLimitReached,
		AlreadySentFoodTwice,
		NoFoodAvailable,
		NotOwner,
		WrongCollectionId,
		NoHatchTimeDetected,
		ShellCollectionIdAlreadySet,
		ShellCollectionIdNotSet,
		RaceNotDetected,
		CareerNotDetected,
		OriginOfShellTypeNotDetected,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
	{
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
			// Ensure that the collection is an Origin of Shell Collection
			ensure!(
				Self::is_origin_of_shell_collection_id(collection_id),
				Error::<T>::WrongCollectionId
			);
			// Ensure that Origin of Shell exists or is not past the hatch time
			let hatch_time = Self::get_hatch_time(collection_id, nft_id);
			ensure!(!Self::can_hatch(hatch_time), Error::<T>::CannotSendFoodToOriginOfShell);
			// Check if account owns an Origin of Shell NFT
			ensure!(
				pallet_uniques::pallet::Pallet::<T>::owned_in_class(&collection_id, &sender)
					.count() > 0,
				Error::<T>::CannotSendFoodToOriginOfShell
			);
			// Get Current Era
			let current_era = pallet_pw_nft_sale::Era::<T>::get();
			// Get number of times fed this era
			let num_of_times_fed =
				OriginOfShellFoodStats::<T>::get((collection_id, nft_id), current_era);

			// Update account FoodInfo if not updated or create new FoodInfo for account's first
			// feeding
			FoodByOwners::<T>::try_mutate(&sender, |food_info| -> Result<(), Error<T>> {
				let mut new_food_info = match food_info {
					None => FoodInfo::new(current_era),
					Some(food_info) if current_era > food_info.era => FoodInfo::new(current_era),
					Some(food_info) => {
						// Ensure sender hasn't fed the Origin of Shell 2 times
						ensure!(
							food_info
								.origin_of_shells_fed
								.iter()
								.filter(|&nft| *nft == (collection_id, nft_id))
								.count() < T::MaxFoodFeedSelf::get().into(),
							Error::<T>::AlreadySentFoodTwice
						);
						food_info.clone()
					},
				};
				new_food_info.origin_of_shells_fed.push((collection_id, nft_id));
				*food_info = Some(new_food_info);

				Ok(())
			})?;

			// Update the Origin of Shell food stats
			OriginOfShellFoodStats::<T>::insert(
				(collection_id, nft_id),
				current_era,
				num_of_times_fed + 1,
			);

			Self::deposit_event(Event::OriginOfShellReceivedFood { collection_id, nft_id, sender });

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
			owner: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_src: BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			pallet_pw_nft_sale::pallet::Pallet::<T>::ensure_overlord(sender.clone())?;
			// Ensure that the collection is an Origin of Shell Collection
			ensure!(
				Self::is_origin_of_shell_collection_id(collection_id),
				Error::<T>::WrongCollectionId
			);
			// Check if HatchTimes is less than or equal to current Timestamp
			// Ensure that Origin of Shell exists or is not past the hatch time
			let hatch_time = Self::get_hatch_time(collection_id, nft_id);
			ensure!(Self::can_hatch(hatch_time), Error::<T>::CannotHatchOriginOfShell);
			// Check if Shell Collection ID is set
			let shell_collection_id = Self::get_shell_collection_id()?;
			// Get race, key and origin of shell type before burning origin of shell NFT
			let race_key = pallet_pw_nft_sale::pallet::Pallet::<T>::to_boundedvec_key("race")?;
			let career_key = pallet_pw_nft_sale::pallet::Pallet::<T>::to_boundedvec_key("career")?;
			let origin_of_shell_type_key =
				pallet_pw_nft_sale::Pallet::<T>::to_boundedvec_key("origin_of_shell_type")?;
			let race = pallet_uniques::Pallet::<T>::attribute(&collection_id, &nft_id, &race_key)
				.ok_or(Error::<T>::RaceNotDetected)?;
			let career =
				pallet_uniques::Pallet::<T>::attribute(&collection_id, &nft_id, &career_key)
					.ok_or(Error::<T>::CareerNotDetected)?;
			let origin_of_shell_type_value = pallet_uniques::Pallet::<T>::attribute(
				&collection_id,
				&nft_id,
				&origin_of_shell_type_key,
			)
			.ok_or(Error::<T>::OriginOfShellTypeNotDetected)?;
			let race_type: RaceType = RaceType::from_u8(race[0]).expect("[race] should not fail");
			let career_type: CareerType =
				CareerType::from_u8(career[0]).expect("[career] should not fail");
			let origin_of_shell_type: OriginOfShellType =
				OriginOfShellType::from_u8(origin_of_shell_type_value[0])
					.expect("[origin_of_shell_type] should not fail");
			// Get Shell Collection next NFT ID
			let shell_nft_id = pallet_rmrk_core::NextNftId::<T>::get(shell_collection_id);
			// Get empty metadata
			let metadata = pallet_pw_nft_sale::pallet::Pallet::<T>::get_empty_metadata();
			// Get resource id
			let resource_id = pallet_rmrk_core::Pallet::<T>::get_next_resource_id()?;
			let resource_id: BoundedResource<T::ResourceSymbolLimit> =
				resource_id.encode().try_into().expect("[resource] id should work");
			// Burn Origin of Shell NFT then Mint Shell NFT
			pallet_rmrk_core::Pallet::<T>::burn_nft(
				Origin::<T>::Signed(owner.clone()).into(),
				collection_id,
				nft_id,
			)?;
			// Mint Shell NFT to Overlord to add attributes and resource before sending to owner
			pallet_rmrk_core::Pallet::<T>::mint_nft(
				origin.clone(),
				owner.clone(),
				shell_collection_id,
				None,
				None,
				metadata,
			)?;
			// Set Origin of Shell Type, Race and Career attributes for NFT
			pallet_pw_nft_sale::Pallet::<T>::set_nft_attributes(
				shell_collection_id,
				shell_nft_id,
				origin_of_shell_type,
				race_type,
				career_type,
			)?;
			// Set the resource for the Shell NFT
			pallet_rmrk_core::Pallet::<T>::add_resource(
				origin.clone(),
				shell_collection_id,
				shell_nft_id,
				resource_id.clone(),
				None,
				Some(resource_src),
				None,
				None,
				None,
				None,
				None,
			)?;
			// Transfer to new owner
			pallet_rmrk_core::Pallet::<T>::accept_resource(
				Origin::<T>::Signed(owner.clone()).into(),
				shell_collection_id,
				shell_nft_id,
				resource_id,
			)?;

			Self::deposit_event(Event::ShellAwakened {
				collection_id: shell_collection_id,
				nft_id: shell_nft_id,
				owner,
			});

			Ok(())
		}

		/// This is an admin function to update origin_of_shells incubation times based on being in
		/// the top 10 of fed origin_of_shells within that era
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic updating the origin_of_shells incubation times
		/// - `origin_of_shells`: Vec of a tuple of Origin of Shells and the time to reduce their
		///   hatch times by
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn update_incubation_time(
			origin: OriginFor<T>,
			origin_of_shells: Vec<((CollectionId, NftId), u64)>,
		) -> DispatchResult {
			// Ensure OverlordOrigin makes call
			let sender = ensure_signed(origin)?;
			pallet_pw_nft_sale::Pallet::<T>::ensure_overlord(sender)?;
			// Iterate through Origin of Shells
			for ((collection_id, nft_id), reduced_time) in origin_of_shells {
				// Ensure that the collection is an Origin of Shell Collection
				ensure!(
					Self::is_origin_of_shell_collection_id(collection_id),
					Error::<T>::WrongCollectionId
				);

				let (old_hatch_time, new_hatch_time) = HatchTimes::<T>::try_mutate(
					collection_id,
					nft_id,
					|hatch_time| -> Result<(u64, u64), Error<T>> {
						let old_hatch_time = match hatch_time {
							None => OfficialHatchTime::<T>::get(),
							Some(hatch_time) => hatch_time.clone(),
						};
						*hatch_time = Some(old_hatch_time.saturating_sub(reduced_time));
						Ok((old_hatch_time, hatch_time.expect("hatch time is ok")))
					},
				)?;
				Self::deposit_event(Event::HatchTimeUpdated {
					collection_id,
					nft_id,
					old_hatch_time,
					new_hatch_time,
				});
			}

			Ok(())
		}

		/// Privileged function to enable incubation phase for accounts to start the incubation
		/// process for their Origin of Shells
		///
		/// Parameters:
		/// `origin`: Expected to be the `Overlord` account
		/// `status`: `bool` value to set for the status in storage
		#[pallet::weight(0)]
		pub fn set_can_start_incubation_status(
			origin: OriginFor<T>,
			status: bool,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account is the sender
			let sender = ensure_signed(origin)?;
			pallet_pw_nft_sale::Pallet::<T>::ensure_overlord(sender)?;
			// Get official hatch time to set in storage
			let start_time = T::Time::now().as_secs();
			let incubation_duration = T::IncubationDurationSec::get();
			let official_hatch_time = start_time + incubation_duration;
			// Set official hatch time
			<OfficialHatchTime<T>>::put(official_hatch_time);
			// Set status in storage
			<CanStartIncubation<T>>::put(status);

			Self::deposit_event(Event::CanStartIncubationStatusChanged {
				status,
				start_time,
				official_hatch_time,
			});

			Ok(Pays::No.into())
		}

		/// Privileged function to set the collection id for the Awakened Shell collection.
		///
		/// Parameters:
		/// - `origin` - Expected Overlord admin account to set the Shell Collection ID
		/// - `collection_id` - Collection ID of the Shell Collection
		#[pallet::weight(0)]
		pub fn set_shell_collection_id(
			origin: OriginFor<T>,
			collection_id: CollectionId,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			pallet_pw_nft_sale::Pallet::<T>::ensure_overlord(sender)?;
			// If Spirit Collection ID is greater than 0 then the collection ID was already set
			ensure!(
				ShellCollectionId::<T>::get().is_none(),
				Error::<T>::ShellCollectionIdAlreadySet
			);
			<ShellCollectionId<T>>::put(collection_id);

			Self::deposit_event(Event::ShellCollectionIdSet { collection_id });

			Ok(Pays::No.into())
		}
	}
}

impl<T: Config> Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	/// Helper function to ensure that the sender owns the origin of shell NFT.
	///
	/// Parameters:
	/// - `sender`: Sender to check if owns the NFT
	/// - `collection_id`: Collection ID of the NFT
	/// - `nft_id`: NFT ID of the NFT
	fn is_owner(sender: &T::AccountId, collection_id: CollectionId, nft_id: NftId) -> bool {
		if let Some(owner) = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id) {
			sender == &owner
		} else {
			// No owner detected return false
			false
		}
	}

	/// Helper function to check the Collection ID matches Origin of Shell Collection ID.
	///
	/// Parameters:
	/// - `collection_id`: Collection ID to check
	fn is_origin_of_shell_collection_id(collection_id: CollectionId) -> bool {
		if let Some(origin_of_shell_collection_id) =
			pallet_pw_nft_sale::OriginOfShellCollectionId::<T>::get()
		{
			collection_id == origin_of_shell_collection_id
		} else {
			false
		}
	}

	/// Helper function to get hatch time has been assigned for an Origin of Shell.
	///
	/// Parameters:
	/// `collection_id`: Collection ID of the Origin of Shell
	/// `nft_id`: NFT ID of the Origin of Shell
	fn get_hatch_time(collection_id: CollectionId, nft_id: NftId) -> u64 {
		let hatch_time = HatchTimes::<T>::get(collection_id, nft_id);
		match hatch_time {
			None => OfficialHatchTime::<T>::get(),
			Some(hatch_time) => hatch_time,
		}
	}

	/// Helper function to check if the Origin of Shell can hatch
	///
	/// Parameters:
	/// `collection_id`: Collection ID of the Origin of Shell
	/// `nft_id`: NFT ID of the Origin of Shell
	fn can_hatch(hatch_time: u64) -> bool {
		let now = T::Time::now().as_secs();
		now > hatch_time
	}

	/// Helper function to get collection id spirit collection
	fn get_shell_collection_id() -> Result<CollectionId, Error<T>> {
		let shell_collection_id =
			ShellCollectionId::<T>::get().ok_or(Error::<T>::ShellCollectionIdNotSet)?;
		Ok(shell_collection_id)
	}
}
