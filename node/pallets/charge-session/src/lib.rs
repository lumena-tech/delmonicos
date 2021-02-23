#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;

use pallet_timestamp as timestamp;

#[derive(Debug, PartialEq, Default, Encode, Decode)]
struct ChargeRequest<ChargerId, Moment> {
  charger_id: ChargerId,
  timestamp: Moment,
}

#[derive(Debug, PartialEq, Default, Encode, Decode)]
struct ChargingSession<ChargerId, Moment> {
  charger_id: ChargerId,
  started_at: Moment,
}

pub trait Config: timestamp::Config {
  type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
  trait Store for Module<T: Config> as ChargeSession {
    // Map of UserAccountId => Latest ChargeRequest
    UserRequests: map hasher(blake2_128_concat) T::AccountId => ChargeRequest<T::AccountId, T::Moment>;
    // Map of UserAccountId => Current ChargingSession
    PendingSessions: map hasher(blake2_128_concat) T::AccountId => ChargingSession<T::AccountId, T::Moment>;
  }
}

decl_event!(
  pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, Moment = <T as timestamp::Config>::Moment {
    // SessionStarted(User, Charger, Timestamp)
    SessionStarted(AccountId, AccountId, Moment),
    // SessionEnded(User, Charger, Timestamp)
    SessionEnded(AccountId, AccountId, Moment),
  }
);

decl_error! {
  pub enum Error for Module<T: Config> {
    NoChargingRequest,
    NoChargingSession,
  }
}

decl_module! {
  pub struct Module<T: Config> for enum Call where origin: T::Origin {
    type Error = Error<T>;

    fn deposit_event() = default;

    #[weight = 1_000]
    pub fn new_request(origin, charger: T::AccountId) -> dispatch::DispatchResult {
      let sender = ensure_signed(origin)?;
      let now = <timestamp::Module<T>>::get();

      // Add the request to the storage with current timestamp
      UserRequests::<T>::insert(&sender, ChargeRequest{ charger_id: charger, timestamp: now });

      Ok(())
    }

    #[weight = 1_000]
    pub fn start_session(origin, user: T::AccountId) -> dispatch::DispatchResult {
      let sender = ensure_signed(origin)?;
      let now = <timestamp::Module<T>>::get();

      // Validate that a request exists for this user & charger
      let request = UserRequests::<T>::get(&user);
      if request.charger_id != sender {
        // Reject the session
        return Err(Error::<T>::NoChargingRequest.into());
      }
      // TODO: check timestamp for maximal period of time between new_request & start_session ?
      // TODO: check that this user does not have another active charging session

      // Remove the request from storage
      UserRequests::<T>::take(&user);

      // Add the pending charging session
      PendingSessions::<T>::insert(&user, ChargingSession{ charger_id: sender.clone(), started_at: now });

      // Emit an event
      Self::deposit_event(RawEvent::SessionStarted(user, sender, now));

      Ok(())
    }

    #[weight = 1_000]
    pub fn end_session(origin, user: T::AccountId) -> dispatch::DispatchResult {
      let sender = ensure_signed(origin)?;
      let now = <timestamp::Module<T>>::get();

      // Validate that a session exists for this user & charger
      let session = PendingSessions::<T>::get(&user);
      if session.charger_id != sender {
        // Reject the session
        return Err(Error::<T>::NoChargingSession.into());
      }
      
      // Remove the request from storage
      PendingSessions::<T>::take(&user);

      // TODO: offchain storage of charging time

      // Emit an event
      Self::deposit_event(RawEvent::SessionEnded(user, sender, now));

      Ok(())
    }

  }
}
