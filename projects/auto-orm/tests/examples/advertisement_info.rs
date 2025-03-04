use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct AdvertisementInfoEntity;

impl sqlx_orm::EntityTrait for AdvertisementInfoEntity {}

#[derive(FromRow)]
pub struct AdvertisementInfo {
    ///
    pub advertise_id: i64,
    ///
    pub channel_id: i64,
    ///
    pub version_id: i64,
    ///
    pub project_id: i64,
    ///
    pub name: String,
    ///
    pub tooltips: String,
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
    pub custom: sqlx::types::JsonValue,
    /// 删除标记, 7 天后才会真正删除
    pub is_deleted: bool,
}

impl AdvertisementInfo {
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
pub struct AdvertisementInfoPk;

impl AdvertisementInfoPk {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<AdvertisementInfo, Error> {
        sqlx::query_as(
            r#"SELECT * FROM "public"."advertisement_info" WHERE "public"."advertisement_info"."advertise_id" = $1 LIMIT 1"#,
        )
        .bind(key)
        .fetch_one(db)
        .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<AdvertisementInfo>, Error> {
        sqlx::query_as(
            r#"SELECT * FROM "public"."advertisement_info" WHERE "public"."advertisement_info"."advertise_id" = ANY($1)"#,
        )
        .bind(key)
        .fetch_all(db)
        .await
    }
}
