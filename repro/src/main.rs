mod database;
#[allow(warnings, unused)]
mod prisma;

use database::{Client, DB};
use prisma::PrismaClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let prisma_client = Arc::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("Error building prisma client"),
    );

    log::info!(
        "{:#?}",
        DB::find_client(prisma_client.clone(), String::from("some-hwid")).await
    );

    log::info!(
        "{:#?}",
        DB::insert_client(
            prisma_client.clone(),
            Client {
                name: "name".to_string(),
                hwid: "hwid".to_string(),
                token: "token".to_string(),
                first_connection: 0u64,
                last_connection: 0u64,
            }
        )
        .await
    );
}
