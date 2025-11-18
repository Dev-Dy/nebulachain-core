use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::chain::{Block, BlockHeader};
use create::storage::Storage;

/// ChainState manages:
/// - current head (hash + height)
/// - inserting validated blocks
/// - interacting with persistent storage
#[derive(Clone)]
pub struct ChainState {
    head: Arc<RwLock<Option<([u8; 32], u64)>>>,
    pub storage: Arc<Storage>,
}

impl ChainState {
    pub async fn new(storage: Arc<Storage>) -> Self {
        let head = None;
        Self {
            head: Arc::new(RwLock::new(head)),
            storage,
        }
    }

    /// Insert a block into persistent storage and update the head.
    /// NOTE: This version assumes all blocks are valid.
    /// Later we will add:
    /// - prev_hash validation
    /// - height checks
    /// - difficulty checks
    /// - chain reorg rules
    pub async fn insert_block(&self, block: Block) -> Result<()> {
        let hash = block.hash();
        let height = block.header.height;

        self.storage.put_block(&hash, &block)?;

        let mut head_guard = self.head.write().await;
        *head_guard = Some((hash, height));

        info!("Stored block: heigth={} hash={:x?}", height, &hash[..6]);
        ok(())
    }

    /// Get current head (hash, height)
    pub async fn get_head(&self) -> Option<([u8; 32], u64)> {
        let guard = self.head.read().await;
        guard.clone()
    }
    /// Lookup a block by hash
    pub fn get_block(&self, hash: &[u8; 32]) -> Result<Option<Block>> {
        self.storage.get_block(hash)
    }

    /// Validate a new block before adding
    pub fn validate_block(&self, block: &Block) -> bool {
        if block.header.height == 0 {
            warn!("Block height is 0 - genesis not implemented yet");
        }
        true
    }
}
