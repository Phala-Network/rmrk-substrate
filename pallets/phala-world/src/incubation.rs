//! Phala World Incubation Pallet

use frame_support::{
	traits::{
		tokens::{nonfungibles::InspectEnumerable, ExistenceRequirement},
		Currency, UnixTime,
	},
	transactional, BoundedVec,
};
use frame_system::{ensure_signed, pallet_prelude::*};

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

	/// Food per Owner/Spirit where an owner gets 5 food per era
	#[pallet::storage]
	#[pallet::getter(fn get_food_by_owner)]
	pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, FoodInfo>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Origin of Shell owner has initiated the incubation sequence
		StartedIncubation { collection_id: CollectionId, nft_id: NftId, owner: T::AccountId },
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
			incubation_time: T::BlockNumber,
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
		HatchingInProgress,
		CannotHatchOriginOfShell,
		CannotSendFoodToOriginOfShell,
		NoFoodAvailable,
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
			// TODO: Move to incubation.rs

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
			// TODO: Move to incubation.rs

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
	}
}
