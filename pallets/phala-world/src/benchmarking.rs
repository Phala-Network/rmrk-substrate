//! Benchmarking setup for pallet-phala-world

use super::*;

use crate::Pallet as PhalaWorld;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin as SystemOrigin;
#[allow(unused)]
use sp_std::prelude::*;

const SEED: u32 = 0;

// fn init_phala_world<T: Config>(CollectionId, CollectionId, T::AccountId)

benchmarks! {
	where_clause { where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>
	}
	set_overlord {
		let caller: T::AccountId = whitelisted_caller();
		let target: T::AccountId = account("target", 0, SEED);
	}: _(SystemOrigin::Root, target)
	// initialize_world_clock {
	// 	let target: T::AccountId = account("target", 0, SEED);
	// }: _(SystemOrigin::Signed(target.clone()))

impl_benchmark_test_suite!(PhalaWorld, crate::mock::ExtBuilder::default().build(whitelisted_caller()), crate::mock::Test);
}
