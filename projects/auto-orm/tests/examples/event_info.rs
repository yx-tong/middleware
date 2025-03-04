use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct EventInfoEntity;

impl sqlx_orm::EntityTrait for EventInfoEntity {}

#[derive(FromRow)]
pub struct EventInfo {
    ///
    pub event_id: i64,
    ///
    pub project_id: Option<i64>,
    /// 数据维度
    pub name: String,
    ///
    pub comment: String,
    ///
    pub group: String,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub create_user: i64,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_user: i64,
    ///
    pub triggers: Vec<sqlx::types::JsonValue>,
    /// 标记为删除, 7 天后才会真正删除
    pub is_deleted: bool,
    ///
    pub custom: sqlx::types::JsonValue,
}

impl EventInfo {
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
pub struct EventMain;

impl EventMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<EventInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."event_info" WHERE "public"."event_info"."event_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<EventInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."event_info" WHERE "public"."event_info"."event_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
