use crate::ItemList;
use sqlx::{Database, Encode, Postgres, Type, encode::IsNull, error::BoxDynError, postgres::PgHasArrayType};

impl<'q, T> Encode<'q, Postgres> for ItemList<T>
where
    for<'a> &'a [T]: Encode<'q, Postgres>,
    T: Encode<'q, Postgres>,
{
    #[inline]
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError> {
        self.list.as_slice().encode_by_ref(buf)
    }
}

impl<T> Type<Postgres> for ItemList<T>
where
    T: PgHasArrayType,
{
    #[inline]
    fn type_info() -> <Postgres as Database>::TypeInfo {
        T::array_type_info()
    }

    #[inline]
    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        T::array_compatible(ty)
    }
}
