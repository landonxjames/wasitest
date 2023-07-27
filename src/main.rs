use aws_credential_types::{provider::ProvideCredentials, Credentials};
use aws_sdk_dynamodb::{config::Region, Client, Error};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let credentials_provider = static_credential_provider();
    let shared_config = aws_config::from_env()
        .region(Region::new("us-east-1"))
        .credentials_provider(credentials_provider)
        .load()
        .await;
    let ddb_client = Client::new(&shared_config);

    let resp = ddb_client.list_tables().send().await?;
    let tables = resp.table_names().unwrap_or_default();
    for table in tables {
        println!("Table Name: {}", table);
    }
    let output = tables.iter().fold("".to_string(), |mut acc, name| {
        acc.push_str(name);
        acc.push_str("\n");
        acc
    });
    println!("{output}");
    Ok(())
}

fn static_credential_provider() -> impl ProvideCredentials {
    Credentials::from_keys("accesskey", "secretkey", Some("sessiontoken".to_string()))
}
