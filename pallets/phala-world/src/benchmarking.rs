//! Benchmarking setup for pallet-phala-world

use super::*;

#[allow(unused)]
use crate::Pallet as PhalaWorld;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use frame_system::RawOrigin as SystemOrigin;


benchmarks! {
	where_clause { where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>
	}
	set_overlord {
		let caller: T::AccountId = whitelisted_caller();
	}: _(SystemOrigin::Root, caller.clone())

impl_benchmark_test_suite!(PhalaWorld, crate::mock::ExtBuilder::default().build(whitelisted_caller()), crate::mock::Test);
}
