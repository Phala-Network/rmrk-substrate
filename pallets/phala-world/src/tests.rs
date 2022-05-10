#![cfg(test)]

use super::*;

use crate::{mock::*, pallet_pw_nft_sale::Error};
use codec::Encode;
use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::Currency, BoundedVec};
use sp_core::{crypto::AccountId32, sr25519, Pair};

use mock::{Call, Event as MockEvent, ExtBuilder, Origin, PWNftSale, Test};
use rmrk_traits::{
	career::CareerType,
	message::{MessageType, OverlordMessage},
	origin_of_shell::OriginOfShellType,
	preorders::PreorderStatus,
	primitives::*,
	race::RaceType,
	status_type::StatusType,
};

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, UniquesStringLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedVec
fn stbk(s: &str) -> BoundedVec<u8, KeyLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a Vec
fn stv(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn metadata_accounts(
	mut alice_metadata: BoundedVec<u8, UniquesStringLimit>,
	mut bob_metadata: BoundedVec<u8, UniquesStringLimit>,
	mut charlie_metadata: BoundedVec<u8, UniquesStringLimit>,
) {
	alice_metadata = stb("I am ALICE");
	bob_metadata = stb("I am BOB");
	charlie_metadata = stb("I am CHARLIE");
}

fn mint_collection(account: AccountId32) {
	// Mint Spirits collection
	RmrkCore::create_collection(Origin::signed(account), bvec![0u8; 20], Some(5), bvec![0u8; 15]);
}

fn mint_spirit(account: AccountId32, spirit_signature: Option<sr25519::Signature>) {
	let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
	if let Some(spirit_signature) = spirit_signature {
		let message =
			OverlordMessage { account: account.clone(), purpose: MessageType::RedeemSpirit };
		let enc_msg = Encode::encode(&message);
		let signature = overlord_pair.sign(&enc_msg);
		assert_ok!(PWNftSale::redeem_spirit(Origin::signed(account), signature, message));
	} else {
		// Mint Spirit NFT
		assert_ok!(PWNftSale::claim_spirit(Origin::signed(account)));
	}
}

fn setup_config(enable_status_type: StatusType) {
	// Set Overlord account
	assert_ok!(PWNftSale::set_overlord(Origin::root(), OVERLORD));
	let spirit_collection_id = RmrkCore::collection_index();
	// Mint Spirits Collection
	mint_collection(OVERLORD);
	// Set Spirit Collection ID
	assert_ok!(PWNftSale::set_spirit_collection_id(Origin::signed(OVERLORD), spirit_collection_id));
	let origin_of_shell_collection_id = RmrkCore::collection_index();
	// Mint Origin of Shells Collection
	mint_collection(OVERLORD);
	// Set Origin of Shell Collection ID
	assert_ok!(PWNftSale::set_origin_of_shell_collection_id(
		Origin::signed(OVERLORD),
		origin_of_shell_collection_id
	));
	// Initialize the Phala World Clock
	assert_ok!(PWNftSale::initialize_world_clock(Origin::signed(OVERLORD)));
	// Initialize Origin of Shell Inventory numbers
	assert_ok!(PWNftSale::init_origin_of_shell_type_counts(Origin::signed(OVERLORD)));
	match enable_status_type {
		StatusType::ClaimSpirits => {
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::ClaimSpirits
			));
		},
		StatusType::PurchaseRareOriginOfShells => {
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::ClaimSpirits
			));
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::PurchaseRareOriginOfShells
			));
		},
		StatusType::PurchasePrimeOriginOfShells => {
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::ClaimSpirits
			));
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::PurchasePrimeOriginOfShells
			));
		},
		StatusType::PreorderOriginOfShells => {
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::ClaimSpirits
			));
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::PreorderOriginOfShells
			));
		},
		StatusType::LastDayOfSale => {
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::ClaimSpirits
			));
			assert_ok!(PWNftSale::set_status_type(
				Origin::signed(OVERLORD),
				true,
				StatusType::LastDayOfSale
			));
		},
	}
}

#[test]
fn claimed_spirit_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		// let overlord_pub = overlord_pair.public();
		// Set Overlord and configuration then enable spirits to be claimed
		setup_config(StatusType::ClaimSpirits);
		let message = OverlordMessage { account: BOB, purpose: MessageType::RedeemSpirit };
		// Sign BOB's Public Key and Metadata encoding with OVERLORD account
		let claim = Encode::encode(&message);
		let overlord_signature = overlord_pair.sign(&claim);
		// Dispatch a redeem_spirit from BOB's account
		assert_ok!(PWNftSale::redeem_spirit(Origin::signed(BOB), overlord_signature, message));
		// ALICE should be able to claim since she has minimum amount of PHA
		assert_ok!(PWNftSale::claim_spirit(Origin::signed(ALICE)));
	});
}

#[test]
fn claimed_spirit_twice_fails() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		//let overlord_pub = overlord_pair.public();
		// Set Overlord and configuration then enable spirits to be claimed
		setup_config(StatusType::ClaimSpirits);
		//  Only root can set the Overlord Admin account
		assert_noop!(PWNftSale::set_overlord(Origin::signed(ALICE), BOB), BadOrigin);
		// Enable spirits to be claimed
		assert_noop!(
			PWNftSale::set_status_type(Origin::signed(BOB), true, StatusType::ClaimSpirits),
			Error::<Test>::RequireOverlordAccount
		);
		// Dispatch a claim spirit from ALICE's account
		assert_ok!(PWNftSale::claim_spirit(Origin::signed(ALICE)));
		// Fail to dispatch a second claim spirit
		assert_noop!(
			PWNftSale::claim_spirit(Origin::signed(ALICE)),
			Error::<Test>::SpiritAlreadyClaimed
		);
	});
}

#[test]
fn start_world_clock_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		// Set the Overlord Admin account
		assert_ok!(PWNftSale::set_overlord(Origin::root(), OVERLORD));
		// Initialize the Phala World Clock
		assert_ok!(PWNftSale::initialize_world_clock(Origin::signed(OVERLORD)));
	});
}

#[test]
fn auto_increment_era_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		// Set Overlord admin as BOB
		assert_ok!(PWNftSale::set_overlord(Origin::root(), BOB));
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OverlordChanged { old_overlord: Some(OVERLORD) },
		));
		// Initialize the Phala World Clock
		assert_ok!(PWNftSale::initialize_world_clock(Origin::signed(BOB)));
		// Check Zero Day is Some(1)
		assert_eq!(PWNftSale::zero_day(), Some(INIT_TIMESTAMP_SECONDS));
		// Go to block 7 that would increment the Era at Block 6
		fast_forward_to(7);
		// Check Era is 1
		assert_eq!(PWNftSale::era(), 1);
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(crate::pallet_pw_nft_sale::Event::NewEra {
			time: 5 * BLOCK_TIME_SECONDS + INIT_TIMESTAMP_SECONDS,
			era: 1,
		}));
		fast_forward_to(16);
		// Check Era is 1
		assert_eq!(PWNftSale::era(), 3);
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(crate::pallet_pw_nft_sale::Event::NewEra {
			time: 15 * BLOCK_TIME_SECONDS + INIT_TIMESTAMP_SECONDS,
			era: 3,
		}));
	});
}

#[test]
fn purchase_rare_origin_of_shell_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		// Set Overlord and configuration then enable purchase of rare origin of shells
		setup_config(StatusType::PurchaseRareOriginOfShells);
		let bob_claim = Encode::encode(&BOB);
		let bob_overlord_signature = overlord_pair.sign(&bob_claim);
		let charlie_claim = Encode::encode(&CHARLIE);
		let charlie_overlord_signature = overlord_pair.sign(&charlie_claim);
		mint_spirit(ALICE, None);
		mint_spirit(BOB, Some(bob_overlord_signature));
		mint_spirit(CHARLIE, Some(charlie_overlord_signature));
		// ALICE purchases Legendary Origin of Shell
		assert_ok!(PWNftSale::buy_rare_origin_of_shell(
			Origin::signed(ALICE),
			OriginOfShellType::Legendary,
			RaceType::AISpectre,
			CareerType::HackerWizard,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Legendary,
				collection_id: 1,
				nft_id: 0,
				owner: ALICE,
				race: RaceType::AISpectre,
				career: CareerType::HackerWizard,
			},
		));
		// BOB tries to buy Legendary Origin of Shell but not enough funds
		assert_noop!(
			PWNftSale::buy_rare_origin_of_shell(
				Origin::signed(BOB),
				OriginOfShellType::Legendary,
				RaceType::Cyborg,
				CareerType::HardwareDruid,
			),
			pallet_balances::Error::<Test>::InsufficientBalance
		);
		// BOB purchases Magic Origin of Shell
		assert_ok!(PWNftSale::buy_rare_origin_of_shell(
			Origin::signed(BOB),
			OriginOfShellType::Magic,
			RaceType::Cyborg,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Magic,
				collection_id: 1,
				nft_id: 1,
				owner: BOB,
				race: RaceType::Cyborg,
				career: CareerType::HardwareDruid,
			},
		));
		// CHARLIE tries to purchase Prime origin of shell and fails
		assert_noop!(
			PWNftSale::buy_rare_origin_of_shell(
				Origin::signed(CHARLIE),
				OriginOfShellType::Prime,
				RaceType::Pandroid,
				CareerType::HackerWizard,
			),
			Error::<Test>::InvalidPurchase
		);
		// CHARLIE purchases Magic Origin Of Shell
		assert_ok!(PWNftSale::buy_rare_origin_of_shell(
			Origin::signed(CHARLIE),
			OriginOfShellType::Magic,
			RaceType::Pandroid,
			CareerType::HackerWizard,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Magic,
				collection_id: 1,
				nft_id: 2,
				owner: CHARLIE,
				race: RaceType::Pandroid,
				career: CareerType::HackerWizard,
			},
		));
		// Check Balances of ALICE and BOB
		assert_eq!(Balances::total_balance(&ALICE), 19_000_000 * PHA);
		assert_eq!(Balances::total_balance(&BOB), 14_000 * PHA);
		assert_eq!(Balances::total_balance(&CHARLIE), 149_000 * PHA);
	});
}

#[test]
fn purchase_prime_origin_of_shell_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		let bob_pair = sr25519::Pair::from_seed(b"09876543210987654321098765432109");
		// let overlord_pub = overlord_pair.public();
		// Set Overlord and configuration then enable spirits to be claimed
		setup_config(StatusType::PurchasePrimeOriginOfShells);
		// Sign BOB's Public Key and Metadata encoding with OVERLORD account
		// Set metadata for buyers
		let mut alice_metadata = BoundedVec::default();
		let mut bob_metadata = BoundedVec::default();
		let mut charlie_metadata = BoundedVec::default();
		metadata_accounts(alice_metadata, bob_metadata.clone(), charlie_metadata);
		let bob_message =
			OverlordMessage { account: BOB, purpose: MessageType::BuyPrimeOriginOfShells };
		let bob_spirit_msg = OverlordMessage { account: BOB, purpose: MessageType::RedeemSpirit };
		// Sign BOB's Public Key and Metadata encoding with OVERLORD account
		let claim = Encode::encode(&bob_message);
		let fake_claim = Encode::encode(&bob_spirit_msg);
		let bob_overlord_signature = overlord_pair.sign(&claim);
		let fake_signature = overlord_pair.sign(&fake_claim);
		// BOB cannot purchase another Origin of Shell without Spirit NFT
		assert_noop!(
			PWNftSale::buy_prime_origin_of_shell(
				Origin::signed(BOB),
				bob_overlord_signature.clone(),
				bob_message.clone(),
				RaceType::AISpectre,
				CareerType::HackerWizard,
			),
			Error::<Test>::MustOwnSpiritToPurchase
		);
		// BOB mints Spirit NFT
		mint_spirit(BOB, None);
		// BOB cannot use RedeemSpirit OverlordMessage to buy prime Origin of Shell
		assert_noop!(
			PWNftSale::buy_prime_origin_of_shell(
				Origin::signed(BOB),
				fake_signature,
				bob_spirit_msg,
				RaceType::AISpectre,
				CareerType::HackerWizard,
			),
			Error::<Test>::WhitelistVerificationFailed
		);
		// BOB purchases a Prime NFT
		assert_ok!(PWNftSale::buy_prime_origin_of_shell(
			Origin::signed(BOB),
			bob_overlord_signature.clone(),
			bob_message.clone(),
			RaceType::AISpectre,
			CareerType::HackerWizard,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Prime,
				collection_id: 1,
				nft_id: 0,
				owner: BOB,
				race: RaceType::AISpectre,
				career: CareerType::HackerWizard,
			},
		));
		// BOB cannot purchase another Origin of Shell
		assert_noop!(
			PWNftSale::buy_prime_origin_of_shell(
				Origin::signed(BOB),
				bob_overlord_signature,
				bob_message,
				RaceType::AISpectre,
				CareerType::HackerWizard,
			),
			Error::<Test>::OriginOfShellAlreadyPurchased
		);
	});
}

#[test]
fn preorder_origin_of_shell_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		// Set Overlord and configuration then enable preorder origin of shells
		setup_config(StatusType::PreorderOriginOfShells);
		mint_spirit(ALICE, None);
		mint_spirit(BOB, None);
		mint_spirit(CHARLIE, None);
		// BOB preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(BOB),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: BOB,
				preorder_id: 0,
			},
		));
		// ALICE preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(ALICE),
			RaceType::Pandroid,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: ALICE,
				preorder_id: 1,
			},
		));
		// Reassign PreorderIndex to max value
		pallet_pw_nft_sale::PreorderIndex::<Test>::mutate(|id| *id = PreorderId::max_value());
		// CHARLIE preorders an origin of shell but max value is reached
		assert_noop!(
			PWNftSale::preorder_origin_of_shell(
				Origin::signed(CHARLIE),
				RaceType::Cyborg,
				CareerType::HackerWizard,
			),
			Error::<Test>::NoAvailablePreorderId
		);
	});
}

#[test]
fn preorder_origin_of_shell_works_2() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		// Set Overlord and configuration then enable preorder origin of shells
		setup_config(StatusType::PreorderOriginOfShells);
		mint_spirit(ALICE, None);
		mint_spirit(BOB, None);
		mint_spirit(CHARLIE, None);
		// BOB preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(BOB),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: BOB,
				preorder_id: 0,
			},
		));
		// ALICE preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(ALICE),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: ALICE,
				preorder_id: 1,
			},
		));
		// Reassign PreorderIndex to max value
		pallet_pw_nft_sale::PreorderIndex::<Test>::mutate(|id| *id = PreorderId::max_value());
		// CHARLIE preorders an origin of shell but max value is reached
		assert_noop!(
			PWNftSale::preorder_origin_of_shell(
				Origin::signed(CHARLIE),
				RaceType::Pandroid,
				CareerType::HackerWizard,
			),
			Error::<Test>::NoAvailablePreorderId
		);
	});
}

#[test]
fn mint_preorder_origin_of_shell_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		// Set Overlord and configuration then enable preorder origin of shells
		setup_config(StatusType::PreorderOriginOfShells);
		mint_spirit(ALICE, None);
		mint_spirit(BOB, None);
		mint_spirit(CHARLIE, None);
		// BOB preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(BOB),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: BOB,
				preorder_id: 0,
			},
		));
		// CHARLIE preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(CHARLIE),
			RaceType::Pandroid,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: CHARLIE,
				preorder_id: 1,
			},
		));
		// ALICE preorders an origin of shell successfully
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(ALICE),
			RaceType::AISpectre,
			CareerType::HackerWizard,
		));
		let preorder_statuses: Vec<(PreorderId, PreorderStatus)> = vec![
			(0u32, PreorderStatus::Chosen),
			(1u32, PreorderStatus::NotChosen),
			(2u32, PreorderStatus::Chosen),
		];
		// Set ALICE & BOB has Chosen and CHARLIE as NotChosen
		assert_ok!(PWNftSale::set_preorder_status(Origin::signed(OVERLORD), preorder_statuses));
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::PreorderResultChanged {
				preorder_id: 2u32,
				status: PreorderStatus::Chosen,
			},
		));
		// Reassign PreorderIndex to max value
		pallet_pw_nft_sale::PreorderIndex::<Test>::mutate(|id| *id = PreorderId::max_value());
		// ALICE preorders an origin of shell but max value is reached
		assert_noop!(
			PWNftSale::preorder_origin_of_shell(
				Origin::signed(ALICE),
				RaceType::Cyborg,
				CareerType::HackerWizard,
			),
			Error::<Test>::NoAvailablePreorderId
		);
		assert_ok!(PWNftSale::set_status_type(
			Origin::signed(OVERLORD),
			false,
			StatusType::PreorderOriginOfShells
		));
		// ALICE claims origin of shells
		assert_ok!(PWNftSale::claim_chosen_preorders(Origin::signed(ALICE)));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Prime,
				collection_id: 1,
				nft_id: 0,
				owner: ALICE,
				race: RaceType::AISpectre,
				career: CareerType::HackerWizard,
			},
		));
		// BOB claims origin of shells
		assert_ok!(PWNftSale::claim_chosen_preorders(Origin::signed(BOB)));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Prime,
				collection_id: 1,
				nft_id: 1,
				owner: BOB,
				race: RaceType::Cyborg,
				career: CareerType::HardwareDruid,
			},
		));
		// CHARLIE should be able to make a call, but the transaction will not trigger an error
		// since all valid preorders are minted and the account could have NotChosen preorders in
		// their storage
		assert_ok!(PWNftSale::claim_chosen_preorders(Origin::signed(CHARLIE)));
		// Check that last event is the same because CHARLIE was NotChosen
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Prime,
				collection_id: 1,
				nft_id: 1,
				owner: BOB,
				race: RaceType::Cyborg,
				career: CareerType::HardwareDruid,
			},
		));
		// Check Balances of ALICE, BOB, CHARLIE & OVERLORD
		assert_eq!(Balances::total_balance(&ALICE), 19_999_990 * PHA);
		assert_eq!(Balances::total_balance(&BOB), 14_990 * PHA);
		assert_eq!(Balances::total_balance(&CHARLIE), 150_000 * PHA);
		assert_eq!(Balances::total_balance(&OVERLORD), 2_813_308_024 * PHA);
	});
}

#[test]
fn claim_refund_preorder_origin_of_shell_works() {
	ExtBuilder::default().build(OVERLORD).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"28133080042813308004281330800428");
		// Set Overlord and configuration then enable preorder origin of shells
		setup_config(StatusType::PreorderOriginOfShells);
		mint_spirit(ALICE, None);
		mint_spirit(BOB, None);
		mint_spirit(CHARLIE, None);
		// BOB preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(BOB),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: BOB,
				preorder_id: 0,
			},
		));
		// CHARLIE preorders an origin of shell
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(CHARLIE),
			RaceType::Pandroid,
			CareerType::HardwareDruid,
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellPreordered {
				owner: CHARLIE,
				preorder_id: 1,
			},
		));
		// ALICE preorders an origin of shell successfully
		assert_ok!(PWNftSale::preorder_origin_of_shell(
			Origin::signed(ALICE),
			RaceType::AISpectre,
			CareerType::HackerWizard,
		));
		// Preorder status Vec
		let preorder_statuses: Vec<(PreorderId, PreorderStatus)> = vec![
			(0u32, PreorderStatus::Chosen),
			(1u32, PreorderStatus::NotChosen),
			(2u32, PreorderStatus::Chosen),
		];
		// Set ALICE & BOB has Chosen and CHARLIE as NotChosen
		assert_ok!(PWNftSale::set_preorder_status(Origin::signed(OVERLORD), preorder_statuses,));
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::PreorderResultChanged {
				preorder_id: 2u32,
				status: PreorderStatus::Chosen,
			},
		));
		// Reassign PreorderIndex to max value
		pallet_pw_nft_sale::PreorderIndex::<Test>::mutate(|id| *id = PreorderId::max_value());
		// ALICE preorders an origin of shell but max value is reached
		assert_noop!(
			PWNftSale::preorder_origin_of_shell(
				Origin::signed(ALICE),
				RaceType::Cyborg,
				CareerType::HackerWizard,
			),
			Error::<Test>::NoAvailablePreorderId
		);
		assert_ok!(PWNftSale::set_status_type(
			Origin::signed(OVERLORD),
			false,
			StatusType::PreorderOriginOfShells
		));
		// ALICE claims origin of shells
		assert_ok!(PWNftSale::claim_chosen_preorders(Origin::signed(ALICE)));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Prime,
				collection_id: 1,
				nft_id: 0,
				owner: ALICE,
				race: RaceType::AISpectre,
				career: CareerType::HackerWizard,
			},
		));
		// BOB claims origin of shells
		assert_ok!(PWNftSale::claim_chosen_preorders(Origin::signed(BOB)));
		// Check if event triggered
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Prime,
				collection_id: 1,
				nft_id: 1,
				owner: BOB,
				race: RaceType::Cyborg,
				career: CareerType::HardwareDruid,
			},
		));
		// CHARLIE should be able to make a call, but the transaction will not trigger an error
		// since all valid preorders are minted and the account could have NotChosen preorders in
		// their storage
		assert_ok!(PWNftSale::claim_chosen_preorders(Origin::signed(CHARLIE)));
		// Check that last event is the same because CHARLIE was NotChosen
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::OriginOfShellMinted {
				origin_of_shell_type: OriginOfShellType::Prime,
				collection_id: 1,
				nft_id: 1,
				owner: BOB,
				race: RaceType::Cyborg,
				career: CareerType::HardwareDruid,
			},
		));
		// CHARLIE still has a reserved balance so he can claim his refund
		assert_ok!(PWNftSale::claim_refund_preorders(Origin::signed(CHARLIE)));
		// Check that last event is the same because CHARLIE was NotChosen
		System::assert_last_event(MockEvent::PWNftSale(
			crate::pallet_pw_nft_sale::Event::RefundWasClaimed {
				preorder_id: 1u32,
				amount: mock::PrimeOriginOfShellPrice::get(),
			},
		));
		// Check Balances of ALICE, BOB, CHARLIE & OVERLORD
		assert_eq!(Balances::total_balance(&ALICE), 19_999_990 * PHA);
		assert_eq!(Balances::total_balance(&BOB), 14_990 * PHA);
		assert_eq!(Balances::total_balance(&CHARLIE), 150_000 * PHA);
		assert_eq!(Balances::total_balance(&OVERLORD), 2_813_308_024 * PHA);
	});
}
