//! Tests cho blog pallet

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pallet::*;
	use frame_support::{
		assert_noop, assert_ok,
		parameter_types,
		traits::{ConstU128, ConstU32, Currency, GenesisBuild, ReservableCurrency},
		PalletId,
	};
	use frame_system as system;
	use sp_core::H256;
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup},
		BuildStorage, Perbill,
	};

	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test {
			System: frame_system,
			Balances: pallet_balances,
			Blog: blog_pallet,
		}
	);

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type RuntimeOrigin = RuntimeOrigin;
		type Nonce = u64;
		type RuntimeCall = RuntimeCall;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Block = Block;
		type RuntimeEvent = RuntimeEvent;
		type BlockHashCount = frame_support::traits::ConstU64<250>;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u128>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	impl pallet_balances::Config for Test {
		type MaxLocks = frame_support::traits::ConstU32<50>;
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type Balance = u128;
		type RuntimeEvent = RuntimeEvent;
		type DustRemoval = ();
		type ExistentialDeposit = frame_support::traits::ConstU128<1>;
		type AccountStore = System;
		type WeightInfo = ();
		type RuntimeHoldReason = ();
		type RuntimeFreezeReason = ();
		type FreezeIdentifier = ();
		type MaxFreezes = ();
		type DoneSlashHandler = ();
	}

	parameter_types! {
		pub const BlogPalletId: PalletId = PalletId(*b"blogpall");
		pub const PostCreationFee: u128 = 10;
		pub const CommentCreationFee: u128 = 1;
		pub const MaxTitleLength: u32 = 200;
		pub const MaxContentLength: u32 = 10_000;
		pub const MaxCommentLength: u32 = 1_000;
		pub const MaxCommentsPerPost: u32 = 100;
	}

	impl blog_pallet::Config for Test {
		type RuntimeEvent = RuntimeEvent;
		type Currency = Balances;
		type PalletId = BlogPalletId;
		type PostCreationFee = PostCreationFee;
		type CommentCreationFee = CommentCreationFee;
		type MaxTitleLength = MaxTitleLength;
		type MaxContentLength = MaxContentLength;
		type MaxCommentLength = MaxCommentLength;
		type MaxCommentsPerPost = MaxCommentsPerPost;
		type WeightInfo = ();
	}

	// Build genesis storage according to the mock runtime.
	pub fn new_test_ext() -> sp_io::TestExternalities {
		let mut storage = frame_system::GenesisConfig::<Test>::default()
			.build_storage()
			.unwrap();

		// Thêm balance cho các test accounts
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(1, 1000),
				(2, 1000),
				(3, 1000),
			],
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		storage.into()
	}

	#[test]
	fn create_post_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Kiểm tra bài viết đã được tạo
			let post = Blog::posts(0).unwrap();
			assert_eq!(post.title, title);
			assert_eq!(post.content, content);
			assert_eq!(post.author, 1);
			assert_eq!(post.id, 0);

			// Kiểm tra event
			System::assert_last_event(
				RuntimeEvent::Blog(blog_pallet::Event::PostCreated {
					post_id: 0,
					author: 1,
					title,
				})
			);
		});
	}

	#[test]
	fn update_post_works() {
		new_test_ext().execute_with(|| {
			let title = b"Original Title".to_vec();
			let content = b"Original Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Cập nhật bài viết
			let new_title = b"Updated Title".to_vec();
			assert_ok!(Blog::update_post(
				RuntimeOrigin::signed(1),
				0,
				Some(new_title.clone()),
				None
			));

			// Kiểm tra đã được cập nhật
			let post = Blog::posts(0).unwrap();
			assert_eq!(post.title, new_title);
			assert_eq!(post.content, content); // Nội dung không đổi
		});
	}

	#[test]
	fn update_post_fails_if_not_author() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết bởi user 1
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// User 2 cố gắng cập nhật - should fail
			assert_noop!(
				Blog::update_post(
					RuntimeOrigin::signed(2),
					0,
					Some(b"New Title".to_vec()),
					None
				),
				Error::<Test>::NotPostAuthor
			);
		});
	}

	#[test]
	fn delete_post_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Xóa bài viết
			assert_ok!(Blog::delete_post(RuntimeOrigin::signed(1), 0));

			// Kiểm tra đã được đánh dấu xóa
			let post = Blog::posts(0).unwrap();
			assert_eq!(post.is_deleted, true);
		});
	}

	#[test]
	fn create_comment_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Tạo bình luận
			let comment_content = b"Great post!".to_vec();
			assert_ok!(Blog::create_comment(
				RuntimeOrigin::signed(2),
				0,
				comment_content.clone()
			));

			// Kiểm tra bình luận đã được tạo
			let comment = Blog::comments(0).unwrap();
			assert_eq!(comment.content, comment_content);
			assert_eq!(comment.post_id, 0);
			assert_eq!(comment.author, 2);
		});
	}

	#[test]
	fn create_comment_fails_if_post_not_found() {
		new_test_ext().execute_with(|| {
			let comment_content = b"Comment".to_vec();

			// Cố gắng bình luận vào bài viết không tồn tại
			assert_noop!(
				Blog::create_comment(
					RuntimeOrigin::signed(1),
					999,
					comment_content
				),
				Error::<Test>::PostNotFound
			);
		});
	}

	#[test]
	fn update_comment_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Tạo bình luận
			let comment_content = b"Original comment".to_vec();
			assert_ok!(Blog::create_comment(
				RuntimeOrigin::signed(2),
				0,
				comment_content.clone()
			));

			// Cập nhật bình luận
			let new_comment = b"Updated comment".to_vec();
			assert_ok!(Blog::update_comment(
				RuntimeOrigin::signed(2),
				0,
				new_comment.clone()
			));

			// Kiểm tra đã được cập nhật
			let comment = Blog::comments(0).unwrap();
			assert_eq!(comment.content, new_comment);
		});
	}

	#[test]
	fn delete_comment_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Tạo bình luận
			let comment_content = b"Comment to delete".to_vec();
			assert_ok!(Blog::create_comment(
				RuntimeOrigin::signed(2),
				0,
				comment_content.clone()
			));

			// Xóa bình luận
			assert_ok!(Blog::delete_comment(RuntimeOrigin::signed(2), 0));

			// Kiểm tra đã được đánh dấu xóa
			let comment = Blog::comments(0).unwrap();
			assert_eq!(comment.is_deleted, true);
		});
	}

	#[test]
	fn title_too_long_fails() {
		new_test_ext().execute_with(|| {
			let title = vec![0u8; 201]; // Vượt quá MaxTitleLength (200)
			let content = b"Content".to_vec();

			assert_noop!(
				Blog::create_post(
					RuntimeOrigin::signed(1),
					title,
					content
				),
				Error::<Test>::TitleTooLong
			);
		});
	}

	#[test]
	fn content_too_long_fails() {
		new_test_ext().execute_with(|| {
			let title = b"Title".to_vec();
			let content = vec![0u8; 10_001]; // Vượt quá MaxContentLength (10_000)

			assert_noop!(
				Blog::create_post(
					RuntimeOrigin::signed(1),
					title,
					content
				),
				Error::<Test>::ContentTooLong
			);
		});
	}
}

