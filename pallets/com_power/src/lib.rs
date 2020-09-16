#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
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

/// personaldata defenition
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PersonalData<BlockNumber> {
    pub name: Vec<u8>,
    pub my_number: Vec<u8>,
    pub birthday: Vec<u8>,
    pub living_address: Vec<u8>,
    pub register_block_number: BlockNumber,
}
pub type PersonalDataOf<T> = PersonalData<<T as frame_system::Trait>::BlockNumber>;

/// communitydata defenition
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CommunityData<BlockNumber, AccountId> {
    pub founder: AccountId,
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub core_member: Vec<AccountId>,
    pub member: Vec<AccountId>,
    pub register_block_number: BlockNumber,
}
pub type CommunityDataOf<T> =
    CommunityData<<T as frame_system::Trait>::BlockNumber, <T as frame_system::Trait>::AccountId>;

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
        /// Just a dummy storage item.
        pub PersonalDatas : map hasher(blake2_128_concat) T::AccountId => Option<PersonalDataOf<T>>;
        pub CommunityDatas : map hasher(blake2_128_concat) Vec<u8> => Option<CommunityDataOf<T>>;
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
        RegistPersonalData(AccountId),
        RegistCommunityData(Vec<u8>, AccountId),
        RegistCoreMemberOfCommunity(Vec<u8>),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// community already created
        CommunityAlreadyCreated,
        /// community is not existed
        CommunityIsNotExisted,
        /// notAllowed
        NotAllowed,
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

        /// regist personal data
        #[weight = 10_000]
        pub fn regist_personal_data(origin,personal_data:PersonalData<<T as frame_system::Trait>::BlockNumber>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let register_block_number = <frame_system::Module<T>>::block_number();
            let r_parsonal_data = PersonalDataOf::<T> {
                name: personal_data.name,
                my_number: personal_data.my_number,
                birthday: personal_data.birthday,
                living_address: personal_data.living_address,
                register_block_number: register_block_number,
            };
            <PersonalDatas<T>>::insert(&who, r_parsonal_data);
            Self::deposit_event(RawEvent::RegistPersonalData(who));
            Ok(())
        }

        /// create community
        #[weight = 10_000]
        pub fn create_community(origin,community_data:CommunityData<<T as frame_system::Trait>::BlockNumber,<T as frame_system::Trait>::AccountId>) -> dispatch::DispatchResult {
            // if the community is already regiseterd, it's error.
            if <CommunityDatas<T>>::get(community_data.name.clone()) != None{
                return Err(Error::<T>::CommunityAlreadyCreated)?;
            }

            let who = ensure_signed(origin)?;
            let register_block_number = <frame_system::Module<T>>::block_number();
            let mut core_member = Vec::new();
            core_member.push(who.clone());
            let r_community_data = CommunityDataOf::<T> {
                founder: who.clone(),
                name: community_data.name.clone(),
                description: community_data.description,
                core_member: core_member,
                member: Vec::new(),
                register_block_number: register_block_number,
            };
            <CommunityDatas<T>>::insert(community_data.name.clone(), r_community_data);
            Self::deposit_event(RawEvent::RegistCommunityData(community_data.name.clone(),who.clone()));
            Ok(())
        }

        /// regist core member of the community
        #[weight = 10_000]
        fn regist_core_member_of_community(origin,name_of_community:Vec<u8>,address_of_member:<T as frame_system::Trait>::AccountId) -> dispatch::DispatchResult {
            let mut community_data;
            // check exist
            match <CommunityDatas<T>>::get(name_of_community.clone()) {
                Some(result) => community_data = result,
                None => return Err(Error::<T>::CommunityIsNotExisted)?,
            };
            // check core member do
            let who = ensure_signed(origin)?;
            let result = community_data.core_member.iter().find(|&s| s == &who);
            if result == None{
                return Err(Error::<T>::NotAllowed)?;
            }
            community_data.core_member.push(address_of_member);
            <CommunityDatas<T>>::insert(community_data.name.clone(), community_data);
            Self::deposit_event(RawEvent::RegistCoreMemberOfCommunity(name_of_community.clone()));
            Ok(())
        }
    }
}
