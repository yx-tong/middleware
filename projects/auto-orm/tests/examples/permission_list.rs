use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct PermissionListEntity;

impl sqlx_orm::EntityTrait for PermissionListEntity {}

#[derive(FromRow)]
pub struct PermissionList {
    ///
    pub permission_id: i64,
}

impl PermissionList {
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
pub struct PermissionMain;

impl PermissionMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<PermissionList, Error> {
        sqlx::query_as(
            r#"SELECT * FROM "public"."permission_list" WHERE "public"."permission_list"."permission_id" = $1 LIMIT 1"#,
        )
        .bind(key)
        .fetch_one(db)
        .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<PermissionList>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."permission_list" WHERE "public"."permission_list"."permission_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
