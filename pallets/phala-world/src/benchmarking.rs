//! Benchmarking setup for pallet-phala-world

use super::*;

#[allow(unused)]
use crate::Pallet as PhalaWorld;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::Origin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	set_overlord {
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
	}: _(Origin::root(), caller)
	verify {
		assert_last_event::<T>(Event::OverlordChanged { old_overlord: None }.into());
	}

	impl_benchmark_test_suite!(PhalaWorld, crate::mock::ExtBuilder::default().build(None), crate::mock::Test);
}
