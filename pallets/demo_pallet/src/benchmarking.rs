// Code based on the Parachain Template provided by Parity Technologies.
// Modified by Group (Nguyễn Đức Hoàng, Trần Đình Việt Huy, Hồ Quốc Long, Nguyễn Bá Thiều Khôi Nguyên)
// for academic purposes (Đồ án Cơ sở – Đại học Đà Lạt, 2025).

use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
	use super::*;
	#[cfg(test)]
	use crate::pallet::Pallet as demo_Pallet;
	use frame_system::RawOrigin;

	#[benchmark]
	fn do_something() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		do_something(RawOrigin::Signed(caller), 100);

		assert_eq!(Something::<T>::get().map(|v| v.block_number), Some(100u32.into()));
	}

	#[benchmark]
	fn cause_error() {
		Something::<T>::put(CompositeStruct { block_number: 100u32.into() });
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));

		assert_eq!(Something::<T>::get().map(|v| v.block_number), Some(101u32.into()));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
