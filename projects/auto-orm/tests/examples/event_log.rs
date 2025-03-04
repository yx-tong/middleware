use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct EventLogEntity;

impl sqlx_orm::EntityTrait for EventLogEntity {}

#[derive(FromRow)]
pub struct EventLog {
    ///
    pub log_id: i64,
    /// 事件发生的事件
    pub time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub event_id: Option<i64>,
    ///
    pub channel_id: Option<i64>,
    ///
    pub version_id: Option<i64>,
    /// 发起操作的玩家
    pub player_id: Option<sqlx::types::Uuid>,
    ///
    pub device: sqlx::types::JsonValue,
    ///
    pub custom: sqlx::types::JsonValue,
}

impl EventLog {
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
pub struct EventLogMain;

impl EventLogMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<EventLog, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."event_log" WHERE "public"."event_log"."log_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<EventLog>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."event_log" WHERE "public"."event_log"."log_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
