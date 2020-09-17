#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
//use primitives::Bytes;

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct PrivateData<BlockNumber> {
    pub my_number: Vec<u8>,
    pub driver_license_number: Vec<u8>,
    pub medical_number: Vec<u8>,
    pub register_block_number: BlockNumber,
}
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct PublicData<BlockNumber> {
    pub name: Vec<u8>,
    pub address: Vec<u8>,
    pub sexality: u8,
    pub job: Vec<u8>,
    pub company: Vec<u8>,
    pub register_block_number: BlockNumber,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ParamPublicData {
    pub name: Vec<u8>,
    pub address: Vec<u8>,
    pub sexality: u8,
    pub job: Vec<u8>,
    pub company: Vec<u8>,
}

impl core::fmt::Debug for ParamPublicData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Hi")
    }
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RosterInfomation<BlockNumber, AccountId> {
    pub address: AccountId,
    pub purpose_of_posting: Vec<u8>,
    pub is_alive: bool,
    pub register_block_number: BlockNumber,
}

//#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash,Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
//pub struct ReverseResolutionOfRoster<AccountId> {
//    pub list_of_roster:Vec<AccountId>,
//}

//#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize,Debug))]
//pub struct ParamRosterInfomation<AccountId> {
//    pub address: AccountId,
//    pub purpose_of_posting: Vec<u8>,
//    pub is_alive:bool,
//}

//impl core::fmt::Debug<frame_system::Trait::AccountId> for ParamRosterInfomation<frame_system::Trait::AccountId> {
//    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//        write!(f, "Hi")
//    }
//}

pub type PublicDataOf<T> = PublicData<<T as frame_system::Trait>::BlockNumber>;
pub type PrivateDataOf<T> = PrivateData<<T as frame_system::Trait>::BlockNumber>;
pub type RosterInfomationOf<T> = RosterInfomation<
    <T as frame_system::Trait>::BlockNumber,
    <T as frame_system::Trait>::AccountId,
>;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        Something get(fn something): Option<u32>;
        pub PublicDatas : map hasher(blake2_128_concat) T::AccountId => Option<PublicDataOf<T>>;
        pub PrivateDatas: map hasher(blake2_128_concat) T::AccountId => Option<PrivateDataOf<T>>;
        pub RosterInfomations: map hasher(blake2_128_concat) T::AccountId => Option<RosterInfomationOf<T>>;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Just a dummy event.
        /// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        /// To emit this event, we call the deposit function, from our runtime functions
        SomethingStored(u32, AccountId),
        RegisterPublicData(AccountId),
        RegisterRosterInfomation(AccountId),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Value was None
        NoneValue,
        /// Value reached maximum and cannot be incremented further
        StorageOverflow,
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;

        /// Regist Public Data
        #[weight = 10_000]
        pub fn register_public_data(origin,public_data:ParamPublicData) -> dispatch::DispatchResult {
            let registerer = ensure_signed(origin)?;
            let register_block_number = <frame_system::Module<T>>::block_number();
            let public_data = PublicDataOf::<T> {
                name:public_data.name,
                address:public_data.address,
                sexality:public_data.sexality,
                job:public_data.job,
                company:public_data.company,
                register_block_number:register_block_number,
            };
            <PublicDatas<T>>::insert(&registerer, public_data);
            Self::deposit_event(RawEvent::RegisterPublicData(registerer));
            Ok(())
        }

        /// Regist Roster Information
        #[weight = 10_000]
        pub fn register_roster_infomation(origin,param_roster_infomation:RosterInfomation<<T as frame_system::Trait>::BlockNumber,<T as frame_system::Trait>::AccountId>) -> dispatch::DispatchResult {
            let registerer = ensure_signed(origin)?;
            let register_block_number = <frame_system::Module<T>>::block_number();
            let roster_info = RosterInfomationOf::<T> {
                address: param_roster_infomation.address,
                purpose_of_posting: param_roster_infomation.purpose_of_posting,
                is_alive: param_roster_infomation.is_alive,
                register_block_number: register_block_number,
            };
            <RosterInfomations<T>>::insert(&registerer, roster_info);
            Self::deposit_event(RawEvent::RegisterRosterInfomation(registerer));
            Ok(())
        }

        /// Sample Function1
        #[weight = 10_000]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
            // Check it was signed and get the signer. See also: ensure_root and ensure_none
            let who = ensure_signed(origin)?;

            // Code to execute when something calls this.
            // For example: the following line stores the passed in u32 in the storage
            Something::put(something);

            // Here we are raising the Something event
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }

        /// Sample Function2
        #[weight = 10_000]
        pub fn cause_error(origin) -> dispatch::DispatchResult {
            // Check it was signed and get the signer. See also: ensure_root and ensure_none
            let _who = ensure_signed(origin)?;

            match Something::get() {
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    Something::put(new);
                    Ok(())
                },
            }
        }
    }
}
