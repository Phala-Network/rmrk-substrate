//! Phala World Incubation Pallet

use frame_support::{
	ensure,
	pallet_prelude::Get,
	traits::{
		tokens::{nonfungibles::InspectEnumerable, ExistenceRequirement},
		Currency, UnixTime,
	},
	transactional, BoundedVec,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use sp_runtime::DispatchResult;

pub use crate::pallet_pw_nft_sale;
pub use pallet_rmrk_core::types::*;
pub use pallet_rmrk_market;

use rmrk_traits::{food::FoodInfo, primitives::*};

pub use self::pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};

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
		type IncubationDuration: Get<u64>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Food per Owner/Spirit where an owner gets 5 food per era
	#[pallet::storage]
	#[pallet::getter(fn get_food_by_owner)]
	pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, FoodInfo>;

	/// Expected hatch Timestamp for an Origin of Shell that started the incubation process
	#[pallet::storage]
	#[pallet::getter(fn get_hatch_time)]
	pub type HatchTime<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, CollectionId, Blake2_128Concat, NftId, u64>;

	/// A bool value to determine if accounts can start incubation of Origin of Shells
	#[pallet::storage]
	#[pallet::getter(fn can_start_incubation)]
	pub type CanStartIncubation<T: Config> = StorageValue<_, bool, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// CanStartIncubation status changed
		CanStartIncubationStatusChanged { status: bool },
		/// Origin of Shell owner has initiated the incubation sequence
		StartedIncubation {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
			start_time: u64,
			hatch_time: u64,
		},
		/// Origin of Shell received food from an account
		OriginOfShellReceivedFood {
			collection_id: CollectionId,
			nft_id: NftId,
			sender: T::AccountId,
			owner: T::AccountId,
		},
		/// A top 10 fed origin_of_shell of the era has updated their incubation time
		HatchTimeUpdated {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
			old_hatch_time: u64,
			new_hatch_time: u64,
		},
		/// An origin_of_shell has been awakened
		OriginOfShellHatched { collection_id: CollectionId, nft_id: NftId, owner: T::AccountId },
		/// Shell has been awakened from an origin_of_shell being hatched and burned
		ShellAwakened {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
			career: u8,
			race: u8,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		StartIncubationNotAvailable,
		HatchingInProgress,
		CannotHatchOriginOfShell,
		CannotSendFoodToOriginOfShell,
		NoFoodAvailable,
		NotOwner,
		WrongCollectionId,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
	{
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
			// Check if IncubationPhase is enabled
			ensure!(CanStartIncubation::<T>::get(), Error::<T>::StartIncubationNotAvailable);
			// Ensure that the collection is an Origin of Shell Collection
			ensure!(
				Self::is_origin_of_shell_collection_id(collection_id),
				Error::<T>::WrongCollectionId
			);
			// Ensure sender is owner
			ensure!(Self::is_owner(sender.clone(), collection_id, nft_id), Error::<T>::NotOwner);
			// Ensure incubation process hasn't been started already
			ensure!(
				!HatchTime::<T>::contains_key(collection_id, nft_id),
				Error::<T>::HatchingInProgress
			);
			// Get time to start hatching process
			let start_time = T::Time::now().as_secs();
			let incubation_duration = T::IncubationDuration::get();
			let hatch_time = start_time + incubation_duration;
			// Update Hatch Time storage
			HatchTime::<T>::insert(collection_id, nft_id, hatch_time);

			Self::deposit_event(Event::StartedIncubation {
				owner: sender,
				collection_id,
				nft_id,
				start_time,
				hatch_time,
			});

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
			// Ensure that the collection is an Origin of Shell Collection
			ensure!(
				Self::is_origin_of_shell_collection_id(collection_id),
				Error::<T>::WrongCollectionId
			);

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
			// TODO: Move to incubation.rs

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
			pallet_pw_nft_sale::Pallet::<T>::ensure_overlord(sender)?;
			// TODO: Move to incubation.rs

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
			// Set status in storage
			<CanStartIncubation<T>>::put(status);

			Self::deposit_event(Event::CanStartIncubationStatusChanged { status });

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
	fn is_owner(sender: T::AccountId, collection_id: CollectionId, nft_id: NftId) -> bool {
		if let Some(owner) = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id) {
			sender == owner
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

	/// Helper function to refill an accounts FoodInfo. This will check if they have updated their
	/// FoodInfo since the last Era that has passed and update the information if needed.
	///
	/// Parameters:
	/// - `sender`: Account owner of the FoodInfo
	/// - `food_count`: Food count for the current Era left
	/// - `food_count_for_self`: Food count to feed to owned Origin of Shells in current Era
	/// - `food_received_this_era`: Food received this current Era
	fn refill_account_food_info(
		sender: T::AccountId,
		food_count: u8,
		food_count_for_self: u8,
	) -> DispatchResult {
		// Check if owner owns an Origin of Shell
		let origin_of_shell_collection_id =
			pallet_pw_nft_sale::OriginOfShellCollectionId::<T>::get()
				.ok_or(pallet_pw_nft_sale::Error::<T>::OriginOfShellCollectionNotSet)?;
		ensure!(
			pallet_pw_nft_sale::Pallet::<T>::owns_nft_in_collection(
				&sender,
				origin_of_shell_collection_id
			),
			Error::<T>::NotOwner
		);
		// Get Current Era
		let current_era = pallet_pw_nft_sale::Era::<T>::get();
		if let Some(mut food_info) = FoodByOwner::<T>::get(sender) {
			// Check if FoodInfo is up to date with current Era
			if current_era > food_info.era {}
		} else {
			// Create FoodInfo for owner
			let new_food_info = FoodInfo {
				era: current_era,
				food_count: T::FoodPerEra::get(),
				food_count_for_self: T::MaxFoodFeedSelf::get(),
			};
		}

		Ok(())
	}

	/// Helper function to create an accounts FoodInfo. This will check if they own an Origin of
	/// Shell NFT then create a FoodInfo type for the account.
	///
	/// Parameters:
	/// - `sender`: Account owner of the FoodInfo
	fn create_account_food_info(sender: T::AccountId) -> DispatchResult {
		// Check if owner owns an Origin of Shell
		let origin_of_shell_collection_id =
			pallet_pw_nft_sale::OriginOfShellCollectionId::<T>::get()
				.ok_or(pallet_pw_nft_sale::Error::<T>::OriginOfShellCollectionNotSet)?;
		ensure!(
			pallet_pw_nft_sale::Pallet::<T>::owns_nft_in_collection(
				&sender,
				origin_of_shell_collection_id
			),
			Error::<T>::NotOwner
		);
		// Create FoodInfo for owner
		let new_food_info = FoodInfo {
			era: pallet_pw_nft_sale::Era::<T>::get(),
			food_count: T::FoodPerEra::get(),
			food_count_for_self: T::MaxFoodFeedSelf::get(),
		};

		Ok(())
	}
}
