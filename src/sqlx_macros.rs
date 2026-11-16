/// Macro to implement sqlx::Type, Decode, and Encode for a type that implements FromStr and
/// Display.
#[macro_export]
macro_rules! impl_sqlx_enum_display_postgres {
    ($t:ty) => {
        impl $crate::sqlx::Type<$crate::sqlx::Postgres> for $t {
            fn type_info() -> $crate::sqlx::postgres::PgTypeInfo {
                <&str as $crate::sqlx::Type<$crate::sqlx::Postgres>>::type_info()
            }
            
            fn compatible(ty: &$crate::sqlx::postgres::PgTypeInfo) -> bool {
                use $crate::sqlx::TypeInfo;
                matches!(ty.name(), "TEXT" | "VARCHAR")
            }
        }

        impl<'r> $crate::sqlx::Decode<'r, $crate::sqlx::Postgres> for $t {
            fn decode(
                value: <$crate::sqlx::Postgres as $crate::sqlx::Database>::ValueRef<'r>,
            ) -> Result<$t, Box<dyn std::error::Error + 'static + Send + Sync>> {
                Ok(<&str as $crate::sqlx::Decode<'r, $crate::sqlx::Postgres>>::decode(value)?.parse()?)
            }
        }

        impl $crate::sqlx::Encode<'_, $crate::sqlx::Postgres> for $t {
            fn encode_by_ref(&self, buf: &mut $crate::sqlx::postgres::PgArgumentBuffer) -> Result<$crate::sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let s = self.to_string();
                <&str as $crate::sqlx::Encode<'_, $crate::sqlx::Postgres>>::encode(&s, buf)
            }
        }
        
        impl $crate::sqlx::postgres::PgHasArrayType for $t {
            // Required method
            fn array_type_info() -> $crate::sqlx::postgres::PgTypeInfo {
                <&str as $crate::sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }
    };
}

/// Macro to implement sqlx::Type, Decode, and Encode for enums with repr(u32) as i32 (Postgres
/// INTEGER).
#[macro_export]
macro_rules! impl_sqlx_enum_repr_postgres {
    ($t:ty) => {
        impl $crate::sqlx::Type<$crate::sqlx::Postgres> for $t {
            fn type_info() -> $crate::sqlx::postgres::PgTypeInfo {
                <i32 as $crate::sqlx::Type<$crate::sqlx::Postgres>>::type_info()
            }
            
            fn compatible(ty: &$crate::sqlx::postgres::PgTypeInfo) -> bool {
                use $crate::sqlx::TypeInfo;
                matches!(ty.name(), "INT4" | "INTEGER")
            }
        }

        impl<'r> $crate::sqlx::Decode<'r, $crate::sqlx::Postgres> for $t {
            fn decode(
                value: <$crate::sqlx::Postgres as $crate::sqlx::Database>::ValueRef<'r>,
            ) -> Result<$t, Box<dyn std::error::Error + 'static + Send + Sync>> {
                let v = <i32 as $crate::sqlx::Decode<'r, $crate::sqlx::Postgres>>::decode(value)?;
                
                <$t as ::std::convert::TryFrom<u32>>::try_from(v as u32)
                    .map_err(|_| format!("Invalid enum value for {}: {}", stringify!($t), v).into())
            }
        }
        
        impl $crate::sqlx::Encode<'_, $crate::sqlx::Postgres> for $t {
            fn encode_by_ref(&self, buf: &mut $crate::sqlx::postgres::PgArgumentBuffer) -> Result<$crate::sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let v = *self as u32 as i32;
                <i32 as $crate::sqlx::Encode<'_, $crate::sqlx::Postgres>>::encode(v, buf)
            }
        }
        
        impl $crate::sqlx::postgres::PgHasArrayType for $t {
            // Required method
            fn array_type_info() -> $crate::sqlx::postgres::PgTypeInfo {
                <i32 as $crate::sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }
    };
}
