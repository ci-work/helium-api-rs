use crate::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Height {
    /// The current block height of the chain.
    pub height: u64,
}

/// Get the current height of the blockchain
pub async fn height(client: &Client) -> Result<u64> {
    let height: Height = client.fetch("/blocks/height", NO_QUERY).await?;
    Ok(height.height)
}

/// Get all the transactions for the account
#[cfg(feature = "transactions")]
pub fn get_block_transactions(client: &Client, height: u64) -> Stream<transactions::Transaction> {
    client
        .fetch_stream(&format!("/blocks/{}/transactions", height), NO_QUERY)
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn heigh() {
        let client = Client::default();
        let height = blocks::height(&client).await.expect("height");
        assert!(height > 0);
    }
}
