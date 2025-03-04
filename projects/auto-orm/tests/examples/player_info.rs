use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct PlayerInfoEntity;

impl sqlx_orm::EntityTrait for PlayerInfoEntity {}

#[derive(FromRow)]
pub struct PlayerInfo {
    ///
    pub player_id: sqlx::types::Uuid,
    ///
    pub external_id: String,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub custom: sqlx::types::JsonValue,
    ///
    pub server_id: Option<i64>,
}

impl PlayerInfo {
    pub async fn find_by(key: String, db: &Pool<Postgres>) -> Result<Self, Error> {
        sqlx::query_as(r#"SELECT * FROM public.advertisement_info WHERE advertise_id = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn find_many(db: &Pool<Postgres>, limit: u64) -> Result<Vec<Self>, Error> {
        sqlx::query_as(r#"SELECT * FROM public.advertisement_info LIMIT $1"#).bind(limit as i64).fetch_all(db).await
    }
}
pub struct PlayerMain;

impl PlayerMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<PlayerInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."player_info" WHERE "public"."player_info"."player_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<PlayerInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."player_info" WHERE "public"."player_info"."player_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
