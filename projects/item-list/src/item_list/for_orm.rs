use super::*;
use sea_orm::sea_query::ValueType;

impl<T> From<ItemList<T>> for sea_orm::Value
where
    T: ValueType,
    T: Into<sea_orm::Value>,
{
    fn from(value: ItemList<T>) -> Self {
        sea_orm::Value::Array(T::array_type(), Some(Box::new(value.list.into_iter().map(Into::into).collect())))
    }
}
