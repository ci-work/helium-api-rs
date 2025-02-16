use futures_util::stream::StreamExt;
use helium_api::{accounts, models::QueryTimeRange, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let account = accounts::get(
        &client,
        "13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH",
    )
    .await?;
    println!("Account: {:?}", account);

    let params = QueryTimeRange {
        min_time: "-30 day".into(),
        max_time: "-1 hour".into(),
    };

    let mut account_activity_stream = accounts::activity(
        &client,
        "13vSgJU5rArGv7SryX9h2n4Rz73LM1Achv1J6eFKgjejoKauPr2",
        &params,
    );

    while let Some(Ok(txn)) = account_activity_stream.next().await {
        println!("{:?}", txn);
    }

    Ok(())
}
