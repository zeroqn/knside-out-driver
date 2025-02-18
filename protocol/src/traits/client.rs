use std::{future::Future, pin::Pin};

use ckb_jsonrpc_types::{
    BlockNumber, BlockView, HeaderView, JsonBytes, OutputsValidator, Transaction,
    TransactionWithStatus,
};
use ckb_sdk::rpc::ckb_indexer::{Cell, Pagination, SearchKey};
use ckb_types::H256;

use crate::{async_trait, KoResult};

#[async_trait]
pub trait CkbClient: Send + Sync + Clone {
    // ckb api
    fn get_block_by_number(&self, number: BlockNumber) -> RPC<BlockView>;

    fn get_block(&self, hash: &H256) -> RPC<BlockView>;

    fn get_tip_header(&self) -> RPC<HeaderView>;

    fn get_transaction(&self, hash: &H256) -> RPC<Option<TransactionWithStatus>>;

    fn send_transaction(
        &self,
        tx: &Transaction,
        outputs_validator: Option<OutputsValidator>,
    ) -> RPC<H256>;

    fn get_txs_by_hashes(&self, hash: Vec<H256>) -> RPC<Vec<Option<TransactionWithStatus>>>;

    // indexer api
    fn fetch_live_cells(
        &self,
        search_key: SearchKey,
        limit: u32,
        cursor: Option<JsonBytes>,
    ) -> RPC<Pagination<Cell>>;
}

pub type RPC<T> = Pin<Box<dyn Future<Output = KoResult<T>> + Send + 'static>>;
