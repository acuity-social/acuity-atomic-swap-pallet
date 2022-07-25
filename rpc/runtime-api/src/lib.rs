#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use sp_std::prelude::Vec;

pub use pallet_acuity_atomic_swap::{AcuityAssetId};

sp_api::decl_runtime_apis! {
    pub trait AtomicSwapApi<AcuityAssetId, AccountId, Balance> where
        AcuityAssetId: codec::Codec,
        AccountId: codec::Codec,
		Balance: codec::Codec,
    {
        fn get_stashes(asset_id: AcuityAssetId, offset: u32, limit: u32) -> sp_std::prelude::Vec<(AccountId, Balance)>;
    }
}