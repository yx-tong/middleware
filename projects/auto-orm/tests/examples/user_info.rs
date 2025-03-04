use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct UserInfoEntity;

impl sqlx_orm::EntityTrait for UserInfoEntity {}

#[derive(FromRow)]
pub struct UserInfo {
    ///
    pub oath_id: sqlx::types::Uuid,
    ///
    pub user_id: i64,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub nick_name: String,
    ///
    pub is_deleted: bool,
}

impl UserInfo {
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
pub struct UserMain;

impl UserMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<UserInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."user_info" WHERE "public"."user_info"."user_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<UserInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."user_info" WHERE "public"."user_info"."user_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
