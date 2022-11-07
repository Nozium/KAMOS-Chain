/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
use support::{
	decl_module, decl_storage, decl_event, StorageValue, dispatch::Result,
	ensure, StorageMap, Parameter
	};
use system::ensure_signed;
use parity_codec::Codec;
use runtime_primitives::traits::{SimpleArithmetic, CheckedSub, CheckedAdd, As};

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type TokenBalance: SimpleArithmetic + Codec + Default + Copy + Parameter + As<u128>;
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as kamostoken {
		// token for AI agent
        Init get(is_init): bool;
        Owner get(owner): T::AccountId;
		
		// Add for manage knowlegtoken
		// Name get(name): Vec<u8>;
		// Ticker get(ticker): Vec<u8>;

        Supply get(supply): T::TokenBalance;
        BalanceOf get(balance_of): map T::AccountId => T::TokenBalance;
	}
}


decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn init(origin,supply: u128) -> Result {
			// check ensure to signed
			let sender = ensure_signed(origin)?;
			ensure!(Self::is_init() == false, "Already initialized.");
			let supply = <T::TokenBalance as As<u128>>::sa(supply);

			// init AI agent as have token. prime as 0
			// let primevalue = <T::TokenBalance as As<u128>>::sa(0);
			<Supply<T>>::put(supply);
			<BalanceOf<T>>::insert(sender.clone(), supply);
			<Owner<T>>::put(sender.clone());
			// <Ticker<T>>::put(ticker);
			// <Name<T>>::put(name);
			<Init<T>>::put(true);
			Ok(())
		}

		fn transfer(_origin, receiver: T::AccountId, value: u128) -> Result {
            let sender = ensure_signed(_origin)?;
            let value = <T::TokenBalance as As<u128>>::sa(value);

            ensure!(<BalanceOf<T>>::exists(sender.clone()),
              "Account does not own this token");
            ensure!(sender != receiver,
              "Account can not send token to yourself");
            let sender_balance = Self::balance_of(sender.clone());
            ensure!(sender_balance >= value, "Not enough balance.");

            let updated_sender_balance = sender_balance - value;
            let receiver_balance = Self::balance_of(receiver.clone());
            let updated_receiver_balance = receiver_balance.checked_add(&value).ok_or("overflow")?;

            <BalanceOf<T>>::insert(sender.clone(), updated_sender_balance);
            <BalanceOf<T>>::insert(receiver.clone(), updated_receiver_balance);

            Ok(())
        }

	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u128, AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type kamostoken = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(kamostoken::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(kamostoken::something(), Some(42));
		});
	}
}
