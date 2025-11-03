//! Weight functions needed for the blog pallet.
//! Sử dụng weight mặc định cho các extrinsics.

use frame_support::weights::{constants::RocksDbWeight, Weight};
use core::marker::PhantomData as CorePhantomData;

/// Weight functions cho blog pallet
pub trait WeightInfo {
	fn create_post(title_len: u32, content_len: u32) -> Weight;
	fn update_post(title_len: u32, content_len: u32) -> Weight;
	fn delete_post() -> Weight;
	fn create_comment(content_len: u32) -> Weight;
	fn update_comment(content_len: u32) -> Weight;
	fn delete_comment() -> Weight;
	fn toggle_post_like() -> Weight;
	fn toggle_comment_like() -> Weight;
	fn add_tags(tags_count: u32) -> Weight;
	fn toggle_bookmark() -> Weight;
	fn toggle_follow() -> Weight;
}

/// SubstrateWeight type alias để dùng trong runtime config
pub struct SubstrateWeight<T>(CorePhantomData<T>);

impl<T> WeightInfo for SubstrateWeight<T> {
	fn create_post(title_len: u32, content_len: u32) -> Weight {
		Weight::from_parts(50_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(title_len as u64 * 1_000, 0))
			.saturating_add(Weight::from_parts(content_len as u64 * 1_000, 0))
	}

	fn update_post(title_len: u32, content_len: u32) -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(title_len as u64 * 1_000, 0))
			.saturating_add(Weight::from_parts(content_len as u64 * 1_000, 0))
	}

	fn delete_post() -> Weight {
		Weight::from_parts(30_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn create_comment(content_len: u32) -> Weight {
		Weight::from_parts(25_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(content_len as u64 * 1_000, 0))
	}

	fn update_comment(content_len: u32) -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(content_len as u64 * 1_000, 0))
	}

	fn delete_comment() -> Weight {
		Weight::from_parts(15_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn toggle_post_like() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn toggle_comment_like() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn add_tags(tags_count: u32) -> Weight {
		Weight::from_parts(25_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(tags_count as u64 * 1_000_000, 0))
	}

	fn toggle_bookmark() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn toggle_follow() -> Weight {
		Weight::from_parts(25_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}

/// Weight implementation cho test và development
impl WeightInfo for () {
	fn create_post(_title_len: u32, _content_len: u32) -> Weight {
		Weight::from_parts(50_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}

	fn update_post(_title_len: u32, _content_len: u32) -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn delete_post() -> Weight {
		Weight::from_parts(30_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn create_comment(_content_len: u32) -> Weight {
		Weight::from_parts(25_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn update_comment(_content_len: u32) -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn delete_comment() -> Weight {
		Weight::from_parts(15_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn toggle_post_like() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn toggle_comment_like() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn add_tags(_tags_count: u32) -> Weight {
		Weight::from_parts(25_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn toggle_bookmark() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn toggle_follow() -> Weight {
		Weight::from_parts(25_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}

