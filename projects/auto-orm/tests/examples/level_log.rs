use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct LevelLogEntity;

impl sqlx_orm::EntityTrait for LevelLogEntity {}

#[derive(FromRow)]
pub struct LevelLog {
    ///
    pub event_id: sqlx::types::Uuid,
    ///
    pub level_id: sqlx::types::Uuid,
    ///
    pub time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub device: sqlx::types::JsonValue,
    ///
    pub stage: sqlx::types::JsonValue,
    ///
    pub is_end: bool,
    /// 自定义信息
    pub custom: sqlx::types::JsonValue,
    ///
    pub team_id: sqlx::types::Uuid,
}

impl LevelLog {
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
pub struct LevelEvent;

impl LevelEvent {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<LevelLog, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."level_log" WHERE "public"."level_log"."event_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<LevelLog>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."level_log" WHERE "public"."level_log"."event_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
