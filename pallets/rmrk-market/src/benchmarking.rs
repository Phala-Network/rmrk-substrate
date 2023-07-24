#![cfg(feature = "runtime-benchmarks")]

// Benchmarks for rmrk-market pallet

use super::*;

use frame_benchmarking::v2::*;
use frame_support::{assert_ok, traits::Get};
use pallet_rmrk_core::Pallet as RmrkCore;
use sp_runtime::{traits::Bounded, Permill};

use crate::Pallet as RmrkMarket;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const SEED: u32 = 0;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn u32_to_balance<T: Config>(
	val: u32,
) -> <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance
{
	<<T as pallet::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance::from(val)
}

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn funded_account<T: Config>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	<T as Config>::Currency::make_free_balance_be(
		&caller,
		BalanceOf::<T>::max_value() / 100u32.into(),
	);

	caller
}

/// Creates a collection
fn create_test_collection<T: Config>(
	caller: T::AccountId,
	collection_index: u32,
) -> CollectionIdOf<T> {
	let collection_id = <T as pallet::Config>::Helper::collection(collection_index);
	let metadata = bvec![0u8; 20];
	let max = None;
	let symbol = bvec![0u8; 15];
	<T as Config>::Currency::make_free_balance_be(
		&caller,
		BalanceOf::<T>::max_value() / 100u32.into(),
	);
	assert_ok!(RmrkCore::<T>::create_collection(
		(RawOrigin::Signed(caller.clone())).into(),
		collection_id.clone(),
		metadata,
		max,
		symbol,
	));
	collection_id
}

/// Mint a token
fn mint_test_nft<T: Config>(
	owner: T::AccountId,
	mint_for: Option<T::AccountId>,
	collection_id: CollectionIdOf<T>,
	nft_index: u32,
) -> ItemIdOf<T> {
	let nft_id = <T as pallet::Config>::Helper::item(nft_index);
	let royalty_recipient = owner.clone();
	let royalty = Permill::from_percent(1);
	let nft_metadata = bvec![0u8; 20];
	let resource = None;
	assert_ok!(RmrkCore::<T>::mint_nft(
		RawOrigin::Signed(owner.clone()).into(),
		mint_for,
		nft_id,
		collection_id,
		Some(royalty_recipient),
		Some(royalty),
		nft_metadata,
		true,
		resource,
	));
	nft_id
}

/// Lists an Nft
fn list_test_nft<T: Config>(
	owner: T::AccountId,
	collection_id: CollectionIdOf<T>,
	nft_id: ItemIdOf<T>,
	price: u32,
) -> <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance
{
	let amount = u32_to_balance::<T>(price);
	assert_ok!(RmrkMarket::<T>::list(
		RawOrigin::Signed(owner.clone()).into(),
		collection_id,
		nft_id,
		amount,
		None,
	));
	amount.into()
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn buy<T: Config>() {
		let owner: T::AccountId = funded_account::<T>("owner", 0);
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(owner.clone(), None, collection_id, 42);
		let price = list_test_nft::<T>(owner.clone(), collection_id, nft_id, 100);
		let caller: T::AccountId = whitelisted_caller();
		<T as Config>::Currency::make_free_balance_be(
			&owner,
			<T as Config>::Currency::minimum_balance(),
		);
		<T as Config>::Currency::make_free_balance_be(
			&caller,
			BalanceOf::<T>::max_value() / 100u32.into(),
		);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), collection_id, nft_id, None);

		assert_last_event::<T>(
			Event::TokenSold { owner, buyer: caller, collection_id, nft_id, price }.into(),
		);
	}

	#[benchmark]
	fn list<T: Config>() {
		let caller: T::AccountId = whitelisted_caller();
		let collection_index = 1;

		let collection_id = create_test_collection::<T>(caller.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(caller.clone(), None, collection_id, 42);
		let price = u32_to_balance::<T>(100);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), collection_id, nft_id, price, None);

		assert_last_event::<T>(
			Event::TokenListed { owner: caller, collection_id, nft_id, price }.into(),
		);
	}

	#[benchmark]
	fn unlist<T: Config>() {
		let caller: T::AccountId = whitelisted_caller();
		let collection_index = 1;

		let collection_id = create_test_collection::<T>(caller.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(caller.clone(), None, collection_id, 42);

		let _ = list_test_nft::<T>(caller.clone(), collection_id, nft_id, 100);
		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), collection_id, nft_id);

		assert_last_event::<T>(
			Event::TokenUnlisted { owner: caller, collection_id, nft_id }.into(),
		);
	}

	#[benchmark]
	fn make_offer<T: Config>() {
		let owner = funded_account::<T>("owner", 0);
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(owner.clone(), None, collection_id, 42);

		let caller: T::AccountId = whitelisted_caller();
		let amount = T::MinimumOfferAmount::get();
		<T as Config>::Currency::make_free_balance_be(
			&caller,
			BalanceOf::<T>::max_value() / 100u32.into(),
		);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), collection_id, nft_id, amount, None);

		assert_last_event::<T>(
			Event::OfferPlaced { offerer: caller, collection_id, nft_id, price: amount }.into(),
		);
	}

	#[benchmark]
	fn withdraw_offer<T: Config>() {
		let owner = funded_account::<T>("owner", 0);
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(owner.clone(), None, collection_id, 42);

		let caller: T::AccountId = whitelisted_caller();
		let amount = T::MinimumOfferAmount::get();
		<T as Config>::Currency::make_free_balance_be(
			&caller,
			BalanceOf::<T>::max_value() / 100u32.into(),
		);

		let _ = RmrkMarket::<T>::make_offer(
			RawOrigin::Signed(caller.clone()).into(),
			collection_id,
			nft_id,
			amount,
			None,
		);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), collection_id, nft_id);

		assert_last_event::<T>(
			Event::OfferWithdrawn { sender: caller, collection_id, nft_id }.into(),
		);
	}

	#[benchmark]
	fn accept_offer<T: Config>() {
		let caller: T::AccountId = whitelisted_caller();
		let collection_index = 1;

		let collection_id = create_test_collection::<T>(caller.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(caller.clone(), None, collection_id, 42);

		let offerer = funded_account::<T>("offerer", 0);
		let amount = T::MinimumOfferAmount::get();
		let _ = RmrkMarket::<T>::make_offer(
			RawOrigin::Signed(offerer.clone()).into(),
			collection_id,
			nft_id,
			amount,
			None,
		);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), collection_id, nft_id, offerer.clone());

		assert_last_event::<T>(
			Event::OfferAccepted { owner: caller, buyer: offerer, collection_id, nft_id }.into(),
		);
	}

	impl_benchmark_test_suite!(RmrkMarket, crate::mock::new_test_ext(), crate::mock::Test);
}
