
/// Macro to implement sqlx::Type, Decode, and Encode for a type that implements FromStr and
/// Display.
#[macro_export]
macro_rules! impl_sqlx_enum_display_postgres {
    ($t:ty) => {
        impl ::sqlx::Type<::sqlx::Postgres> for $t {
            fn type_info() -> ::sqlx::postgres::PgTypeInfo {
                <&str as ::sqlx::Type<::sqlx::Postgres>>::type_info()
            }
            
            fn compatible(ty: &::sqlx::postgres::PgTypeInfo) -> bool {
                use sqlx::TypeInfo;
                matches!(ty.name(), "TEXT" | "VARCHAR")
            }
        }

        impl<'r> ::sqlx::Decode<'r, ::sqlx::Postgres> for $t {
            fn decode(
                value: <::sqlx::Postgres as ::sqlx::Database>::ValueRef<'r>,
            ) -> Result<$t, Box<dyn std::error::Error + 'static + Send + Sync>> {
                Ok(<&str as ::sqlx::Decode<'r, ::sqlx::Postgres>>::decode(value)?.parse()?)
            }
        }

        impl ::sqlx::Encode<'_, ::sqlx::Postgres> for $t {
            fn encode_by_ref(&self, buf: &mut ::sqlx::postgres::PgArgumentBuffer) -> Result<::sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let s = self.to_string();
                <&str as ::sqlx::Encode<'_, ::sqlx::Postgres>>::encode(&s, buf)
            }
        }
        
        impl sqlx::postgres::PgHasArrayType for $t {
            // Required method
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <&str as sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }
    };
}

/// Macro to implement sqlx::Type, Decode, and Encode for enums with repr(u32) as i32 (Postgres
/// INTEGER).
#[macro_export]
macro_rules! impl_sqlx_enum_repr_postgres {
    ($t:ty) => {
        impl ::sqlx::Type<::sqlx::Postgres> for $t {
            fn type_info() -> ::sqlx::postgres::PgTypeInfo {
                <i32 as ::sqlx::Type<::sqlx::Postgres>>::type_info()
            }
            
            fn compatible(ty: &::sqlx::postgres::PgTypeInfo) -> bool {
                use sqlx::TypeInfo;
                matches!(ty.name(), "INT4" | "INTEGER")
            }
        }

        impl<'r> ::sqlx::Decode<'r, ::sqlx::Postgres> for $t {
            fn decode(
                value: <::sqlx::Postgres as ::sqlx::Database>::ValueRef<'r>,
            ) -> Result<$t, Box<dyn std::error::Error + 'static + Send + Sync>> {
                let v = <i32 as ::sqlx::Decode<'r, ::sqlx::Postgres>>::decode(value)?;
                
                <$t as ::std::convert::TryFrom<u32>>::try_from(v as u32)
                    .map_err(|_| format!("Invalid enum value for {}: {}", stringify!($t), v).into())
            }
        }
        
        impl ::sqlx::Encode<'_, ::sqlx::Postgres> for $t {
            fn encode_by_ref(&self, buf: &mut ::sqlx::postgres::PgArgumentBuffer) -> Result<::sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let v = *self as u32 as i32;
                <i32 as ::sqlx::Encode<'_, ::sqlx::Postgres>>::encode(v, buf)
            }
        }
        
        impl sqlx::postgres::PgHasArrayType for $t {
            // Required method
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <i32 as sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }
    };
}
