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
    pub driver_license_number: sp_std::vec::Vec<u8>,
    pub medical_number: sp_std::vec::Vec<u8>,
    pub register_block_number: BlockNumber,
}
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct PublicData<BlockNumber> {
    pub name: sp_std::vec::Vec<u8>,
    pub address: sp_std::vec::Vec<u8>,
    pub sexality: u8,
    pub job: sp_std::vec::Vec<u8>,
    pub company: sp_std::vec::Vec<u8>,
    pub register_block_number: BlockNumber,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ParamPublicData {
    pub name: sp_std::vec::Vec<u8>,
    pub address: sp_std::vec::Vec<u8>,
    pub sexality: u8,
    pub job: sp_std::vec::Vec<u8>,
    pub company: sp_std::vec::Vec<u8>,
}

impl core::fmt::Debug for ParamPublicData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Hi")
    }
}

pub type PublicDataOf<T> = PublicData<<T as frame_system::Trait>::BlockNumber>;
pub type PrivateDataOf<T> = PrivateData<<T as frame_system::Trait>::BlockNumber>;

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
        pub PublicDatas get(fn puglic_datas): map hasher(blake2_128_concat) T::AccountId => Option<PublicDataOf<T>>;
        pub PrivateDatas: map hasher(blake2_128_concat) T::AccountId => Option<PrivateDataOf<T>>;
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


        /// regist public data
//        #[weight = 10_000]
//        pub fn register_public_data(origin,name:sp_std::vec::Vec<u8>,address:sp_std::vec::Vec<u8>,sexality:u8,job:sp_std::vec::Vec<u8>,company:sp_std::vec::Vec<u8>) -> dispatch::DispatchResult {
//            let registerer = ensure_signed(origin)?;
//            let register_block_number = <frame_system::Module<T>>::block_number();
//            let public_data = PublicDataOf::<T> {
//                name,
//                address,
//                sexality,
//                job,
//                company,
//                register_block_number,
//            };
//            <PublicDatas<T>>::insert(&registerer, public_data);
//            Ok(())
//        }
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
            Ok(())
        }

        /// Just a dummy entry point.
        /// function that can be called by the external world as an extrinsics call
        /// takes a parameter of the type `AccountId`, stores it, and emits an event
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

        /// Another dummy entry point.
        /// takes no parameters, attempts to increment storage value, and possibly throws an error
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
