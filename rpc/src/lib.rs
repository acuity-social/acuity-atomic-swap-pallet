use codec::Codec;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::Block as BlockT,
};

use std::sync::Arc;

pub use pallet_acuity_atomic_swap_rpc_runtime_api::AtomicSwapApi as AtomicSwapRuntimeApi;
pub use pallet_acuity_atomic_swap::AcuityAssetId;

#[rpc(client, server)]
pub trait AtomicSwapApi<AcuityAssetId, AccountId, BlockNumber, BlockHash> {
	#[method(name = "atomicSwap_getIndexBlocks")]
	fn get_index_blocks(&self, account: AccountId, at: Option<BlockHash>) -> RpcResult<Vec<BlockNumber>>;
}

pub struct AtomicSwap<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> AtomicSwap<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

#[async_trait]
impl<C, AccountId, Block, BlockNumber>
	AtomicSwapApiServer<AcuityAssetId, AccountId, BlockNumber, <Block as BlockT>::Hash>
	for AtomicSwap<C, Block>
where
    AccountId: Codec,
    Block: BlockT,
	BlockNumber: Codec + Copy + Send + Sync + 'static,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: AtomicSwapRuntimeApi<Block, AccountId, BlockNumber>,
{
	fn get_index_blocks(
        &self,
		account: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Vec<BlockNumber>> {
    	let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

		api.get_index_blocks(&at, account).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query dispatch info.",
				Some(e.to_string()),
			))
			.into()
		})
	}
}
