use std::convert::TryInto;
use std::time::Duration;

use web3::futures::TryFutureExt;
use web3::types::{BlockNumber, FilterBuilder, SyncState};

#[tokio::main]
async fn main() {
    // Start future log stream before requesting current block number, to ensure BlockNumber::Pending isn't after current_block

    let node_endpoint = "wss://rinkeby.infura.io/ws/v3/8225b8de4cc94062959f38e0781586d1";
    let web3 = tokio::time::timeout(Duration::from_secs(5), async {
        Ok(web3::Web3::new(
            web3::transports::WebSocket::new(node_endpoint).await?,
        ))
    })
    // Flatten the Result<Result<>> returned by timeout()
    .map_err(|error| anyhow::Error::new(error))
    .and_then(|x| async { x })
    // Make sure the eth node is fully synced
    .and_then(|web3| async {
        while let SyncState::Syncing(_) = web3.eth().syncing().await? {
            tokio::time::sleep(Duration::from_secs(4)).await;
        }
        Ok(web3)
    })
    .await
    .unwrap();
    let uni_v2_contract: [u8; 20] = hex::decode("7a250d5630B4cF539739dF2C5dAcb4c659F2488D")
        .unwrap()
        .try_into()
        .unwrap();
    web3.eth_subscribe()
        .subscribe_logs(
            FilterBuilder::default()
                // change this to BlockNumber::Latest and it works
                .from_block(BlockNumber::Pending)
                .address(vec![uni_v2_contract.into()])
                .build(),
        )
        .await
        .unwrap();
}
