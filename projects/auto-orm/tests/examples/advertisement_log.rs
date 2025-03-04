use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct AdvertisementLogEntity;

impl sqlx_orm::EntityTrait for AdvertisementLogEntity {}

#[derive(FromRow)]
pub struct AdvertisementLog {
    ///
    pub log_id: i64,
    ///
    pub advertise_id: Option<i64>,
    ///
    pub utm_source: String,
    ///
    pub utm_medium: String,
    ///
    pub utm_campaign: String,
    ///
    pub utm_content: String,
    ///
    pub utm_term: String,
    /// 事件发生时间
    pub time: sqlx::types::chrono::NaiveDateTime,
    /// 用户 IP 地址
    pub ip: String,
    /// 设备信息
    pub user_agent: String,
    ///
    pub callback: String,
    ///
    pub callback_used: bool,
    ///
    pub custom: sqlx::types::JsonValue,
}

impl AdvertisementLog {
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
pub struct AdvertisementLogPk;

impl AdvertisementLogPk {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<AdvertisementLog, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."advertisement_log" WHERE "public"."advertisement_log"."log_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<AdvertisementLog>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."advertisement_log" WHERE "public"."advertisement_log"."log_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
