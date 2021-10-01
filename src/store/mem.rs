use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use webb::evm::ethers::types;

use super::{HistoryStore, LeafCacheStore};

type MemStore = HashMap<types::Address, Vec<(u32, types::H256)>>;

#[derive(Clone, Default)]
pub struct InMemoryStore {
    store: Arc<RwLock<MemStore>>,
    last_block_numbers: Arc<RwLock<HashMap<types::Address, types::U64>>>,
}

impl std::fmt::Debug for InMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryStore").finish()
    }
}

impl HistoryStore for InMemoryStore {
    #[tracing::instrument(skip(self))]
    fn get_last_block_number(
        &self,
        contract: types::Address,
        default_block_number: types::U64,
    ) -> anyhow::Result<types::U64> {
        let guard = self.last_block_numbers.read();
        let val = guard
            .get(&contract)
            .cloned()
            .unwrap_or(default_block_number);
        Ok(val)
    }

    #[tracing::instrument(skip(self))]
    fn set_last_block_number(
        &self,
        contract: types::Address,
        block_number: types::U64,
    ) -> anyhow::Result<types::U64> {
        let mut guard = self.last_block_numbers.write();
        let val = guard.entry(contract).or_insert(block_number);
        let old = *val;
        *val = block_number;
        Ok(old)
    }
}

impl LeafCacheStore for InMemoryStore {
    type Output = Vec<types::H256>;

    #[tracing::instrument(skip(self))]
    fn get_leaves(
        &self,
        contract: types::Address,
    ) -> anyhow::Result<Self::Output> {
        let guard = self.store.read();
        let val = guard
            .get(&contract)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|v| v.1)
            .collect();
        Ok(val)
    }

    #[tracing::instrument(skip(self))]
    fn insert_leaves(
        &self,
        contract: types::Address,
        leaves: &[(u32, types::H256)],
    ) -> anyhow::Result<()> {
        let mut guard = self.store.write();
        guard
            .entry(contract)
            .and_modify(|v| v.extend_from_slice(leaves))
            .or_insert_with(|| leaves.to_vec());
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn get_last_deposit_block_number(
        &self,
        contract: types::Address,
    ) -> anyhow::Result<types::U64> {
        Ok(types::U64::from(0))
    }

    #[tracing::instrument(skip(self))]
    fn insert_last_deposit_block_number(
        &self,
        contract: types::Address,
        block_number: types::U64,
    ) -> anyhow::Result<types::U64> {
        Ok(types::U64::from(0))
    }
}