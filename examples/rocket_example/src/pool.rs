use async_trait::async_trait;
use rocket_db_pools::{rocket::figment::Figment, Config};

#[derive(Debug)]
pub struct RocketDbPool {
    pub db_url: String,
}

#[async_trait]
impl rocket_db_pools::Pool for RocketDbPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let db_url = config.url;

        Ok(RocketDbPool {
            db_url: db_url.to_owned(),
        })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(sea_orm::Database::connect(&self.db_url).await.unwrap())
    }
}
