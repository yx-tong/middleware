use sqlx::{Database, Error, FromRow, IntoArguments, Pool, Postgres, Row};
use sqlx_core::encode::{Encode, IsNull};

pub struct ProjectChannelEntity;

impl sqlx_orm::EntityTrait for ProjectChannelEntity {}

#[derive(FromRow)]
pub struct ProjectChannel {
    ///
    pub channel_id: i64,
    ///
    pub project_id: Option<i64>,
    ///
    pub version_id: Option<i64>,
    ///
    pub name: String,
    ///
    pub comment: String,
    ///
    pub group: String,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub create_user: Option<i64>,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_user: Option<i64>,
    /// 删除标记, 保留 7 天后才真正删除
    pub is_deleted: bool,
    ///
    pub custom: sqlx::types::JsonValue,
}

#[derive(FromRow)]
pub struct ProjectChannelActivity {
    ///
    pub channel_id: i64,
    ///
    pub project_id: Option<i64>,
    ///
    pub version_id: Option<i64>,
    ///
    pub name: Option<String>,
    ///
    pub comment: Option<String>,
    ///
    pub group: Option<String>,
}

impl ProjectChannelEntity {
    pub fn create(
        // must provide args
        must: i64,
        project_id: i64,
        name: String,
    ) -> ProjectChannelActivity {
        ProjectChannelActivity {
            //
            name: Some(name),
            project_id: Some(project_id),
            channel_id: must,
            version_id: None,
            comment: None,
            group: None,
        }
    }
    pub fn read(key: i64, db: &Pool<Postgres>) -> Result<ProjectChannel, Error> {
        GameServerMain::find::<ProjectChannel>(key, db).await
    }
    pub fn update(key: i64, db: &Pool<Postgres>) -> ProjectChannelActivity {
        GameServerMain::find::<ProjectChannelActivity>(key, db).await
    }

    pub fn delete(key: i64, db: &Pool<Postgres>) {
        ProjectChannelActivity {
            //
            name: Some(name),
            project_id: None,
            channel_id: None,
            version_id: None,
            comment: None,
            group: None,
        }
    }
}

impl ProjectChannelActivity {
    pub async fn insert(self, db: &Pool<Postgres>) -> Result<u64, Error> {
        let result = sqlx::query(
            r#"INSERT (
        
        
        )"#,
        )
        .bind(self.channel_id)
        .execute(db)
        .await?;
        Ok(result.rows_affected())
    }
    pub fn update() {}
    pub fn with_project_id(mut self, v: i64) -> Self {
        self.project_id = Some(v);
        self
    }
    pub fn skip_project_id(mut self) -> Self {
        self.project_id = None;
        self
    }
}
pub struct GameServerMain;

impl GameServerMain {
    // Create, read, update and delete
    pub async fn find<T>(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<T, Error>
    where
        T: Unpin + Send,
        T: for<'r> FromRow<'r, <Postgres as Database>::Row>,
    {
        sqlx::query_as(r#"SELECT * FROM "public"."project_channel" WHERE "public"."project_channel"."channel_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn find_all<T>(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<T>, Error>
    where
        T: Unpin + Send,
        T: for<'r> FromRow<'r, <Postgres as Database>::Row>,
    {
        sqlx::query_as(r#"SELECT * FROM "public"."project_channel" WHERE "public"."project_channel"."channel_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
