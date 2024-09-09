use sea_orm::{ActiveValue, DbErr, EntityTrait, TransactionTrait};
use sea_orm::prelude::{Uuid};
use yx_orm::{NaiveDateTime, users};
use crate::database::YxDatabase;
// Change this according to your aliyun_oss implementation,
// or supply it as an environment variable.
// the whole aliyun_oss URL string follows the following format:
// "protocol://username:password@host:port/aliyun_oss"
// We put the aliyun_oss name (that last bit) in a separate variable simply for convenience.


impl YxDatabase {
    pub async fn register_with_phone(&self, phone: i32) -> Result<(), DbErr> {
        let mut task = self.transaction().await?;
        let user_id = Uuid::new_v4();
        let new_user = users::core::ActiveModel {
            user: ActiveValue::Set(user_id),
            created_at: ActiveValue::Set(NaiveDateTime::default()),
            last_login: ActiveValue::Set(NaiveDateTime::default()),
        };
        let _ = users::core::Entity::insert(new_user).exec(&task).await?;

        let new_phone = users::phone::ActiveModel {
            phone: ActiveValue::Set(phone),
            user: ActiveValue::Set(user_id),
            verification_code: Default::default(),
            verification_code_expired: Default::default(),
        };
        let _ = users::phone::Entity::insert(new_phone).exec(&task).await?;
        task.commit().await
    }
}


#[tokio::test]
async fn main() {
    let db = YxDatabase::connect().await.unwrap();
    db.register_with_phone(114514).await.unwrap()
}