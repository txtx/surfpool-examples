use crate::prelude::{Result};
use crate::prelude::*;

pub struct RpcHelper {
    client: Arc<RpcClient>,
    latest_blockhash: Arc<LatestBlockhash>,
}

struct LatestBlockhash {
    blockhash: RwLock<Hash>,
    slot: AtomicU64,
}

impl RpcHelper {
    pub fn new(rpc_url: &str) -> Self {
        let client = Arc::new(RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        ));

        let latest_blockhash = Arc::new(LatestBlockhash {
            blockhash: RwLock::new(Hash::default()),
            slot: AtomicU64::new(0),
        });

        // Start background task to update blockhash
        let client_clone = client.clone();
        let blockhash_clone = latest_blockhash.clone();
        tokio::spawn(async move {
            loop {
                if let Ok((blockhash, slot)) = client_clone
                    .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
                {
                    let mut blockhash_write = blockhash_clone.blockhash.write().await;
                    *blockhash_write = blockhash;
                    blockhash_clone.slot.store(slot, Ordering::Relaxed);
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });

        Self {
            client,
            latest_blockhash,
        }
    }

    pub async fn send_transaction(&self, tx: Transaction) -> Result<Signature> {
        // Wait for initial blockhash
        loop {
            let slot = self.latest_blockhash.slot.load(Ordering::Relaxed);
            if slot != 0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        self.client
            .send_transaction(&tx)
            .map_err(|e| e.into())
    }

    pub async fn get_latest_blockhash(&self) -> Hash {
        *self.latest_blockhash.blockhash.read().await
    }
}
