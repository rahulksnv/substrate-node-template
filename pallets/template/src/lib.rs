#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

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
		UserAdded { who: T::AccountId },
		/// User removed.
		UserRemoved { who: T::AccountId },
		/// User info changed.
		UserInfoChanged { who: T::AccountId, x: i128, y: i128},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// User already exists.
		UserExists,
		/// User not found.
		UserNotFound,
	}

	#[pallet::storage]
	pub(super) type UserMap<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (T::BlockNumber, Option<(i128, i128)>)>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_user(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!UserMap::<T>::contains_key(&sender), Error::<T>::UserExists);

			let block_number = <frame_system::Pallet<T>>::block_number();
			let val: (T::BlockNumber, Option<(i128, i128)>) = (block_number, None);
			UserMap::<T>::insert(&sender, val);
			Self::deposit_event(Event::UserAdded { who: sender });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn remove_user(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(UserMap::<T>::contains_key(&sender), Error::<T>::UserNotFound);

			UserMap::<T>::remove(&sender);
			Self::deposit_event(Event::UserRemoved { who: sender });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn update_user_info(origin: OriginFor<T>, x: i128, y: i128) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(UserMap::<T>::contains_key(&sender), Error::<T>::UserNotFound);

			let block_number = <frame_system::Pallet<T>>::block_number();
			let val: (T::BlockNumber, Option<(i128, i128)>) = (block_number, Some((x, y)));
			UserMap::<T>::set(&sender, Some(val));
			Self::deposit_event(Event::UserInfoChanged { who: sender, x, y });
			Ok(())
		}
	}
}


