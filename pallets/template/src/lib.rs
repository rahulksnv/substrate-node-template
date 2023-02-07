#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::UniqueSaturatedInto;
	use frame_system::pallet_prelude::*;

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq, Clone)]
	pub struct UserState {
		/// Last block number of UserAdded/UserInfoChanged event for the user.
		last_updated_block: u64,

		/// The user coordinates at the last UserAdded/UserInfoChanged event.
		x: u64,
		y: u64,
	}

	impl UserState {
		pub fn new(last_updated_block: u64, x: u64, y: u64) -> Self {
			Self { last_updated_block, x, y }
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// User added.
		UserAdded { who: T::AccountId, user_state: UserState },
		/// User removed.
		UserRemoved { who: T::AccountId, user_state: UserState },
		/// User info changed.
		UserInfoChanged { who: T::AccountId, user_state: UserState },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// User already exists.
		UserExists,
		/// User not found.
		UserNotFound,
	}

	#[pallet::storage]
	pub(super) type UserStateMap<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserState>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_user(origin: OriginFor<T>, x: u64, y: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!UserStateMap::<T>::contains_key(&sender), Error::<T>::UserExists);

			let block_number = <frame_system::Pallet<T>>::block_number().unique_saturated_into();
			let user_state = UserState::new(block_number, x, y);
			UserStateMap::<T>::insert(&sender, user_state.clone());
			Self::deposit_event(Event::UserAdded { who: sender, user_state });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn remove_user(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(UserStateMap::<T>::contains_key(&sender), Error::<T>::UserNotFound);

			let block_number = <frame_system::Pallet<T>>::block_number().unique_saturated_into();
			let mut user_state = UserStateMap::<T>::get(sender.clone()).unwrap();
			user_state.last_updated_block = block_number;
			UserStateMap::<T>::remove(&sender);
			Self::deposit_event(Event::UserRemoved { who: sender, user_state });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn update_user_info(origin: OriginFor<T>, x: u64, y: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(UserStateMap::<T>::contains_key(&sender), Error::<T>::UserNotFound);

			let mut user_state = UserStateMap::<T>::get(sender.clone()).unwrap();
			let block_number = <frame_system::Pallet<T>>::block_number().unique_saturated_into();
			user_state.last_updated_block = block_number;
			user_state.x = x;
			user_state.y = y;
			UserStateMap::<T>::set(sender.clone(), Some(user_state.clone()));
			Self::deposit_event(Event::UserInfoChanged { who: sender, user_state });
			Ok(())
		}
	}
}
