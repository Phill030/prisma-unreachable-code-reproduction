use std::sync::Arc;

use crate::prisma::{
    clients::{self, Data},
    PrismaClient,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub struct Client {
    pub name: String,
    pub hwid: String,
    pub token: String,
    pub first_connection: u64,
    pub last_connection: u64,
}

pub struct DB;
impl DB {
    pub async fn find_client(prisma_client: Arc<PrismaClient>, hwid: String) -> Option<Data> {
        let result = prisma_client
            .clients()
            .find_unique(clients::hwid::equals(hwid))
            .exec()
            .await
            .unwrap();

        result
    }

    pub async fn insert_client(prisma_client: Arc<PrismaClient>, c: Client) -> () {
        prisma_client
            .clients()
            .create(
                c.hwid,
                c.token,
                c.name,
                c.first_connection.try_into().unwrap(),
                c.last_connection.try_into().unwrap(),
                vec![],
            )
            .exec()
            .await
            .unwrap();
    }
}
