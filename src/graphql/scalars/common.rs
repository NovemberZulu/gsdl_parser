macro_rules! enumerable_enum {
    ($name:ident { $($value:ident,)* } ) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $name { $($value),* }

        use std::slice::Iter;
        impl $name {
            const ENUM_ITEMS: &'static [($name, &'static str)] =
                &[$(($name::$value, stringify!($value))),*];

            pub fn iter() -> Iter<'static, ($name, &'static str)> {
                $name::ENUM_ITEMS.into_iter()
            }

            pub fn count() -> usize {
                $name::ENUM_ITEMS.len()
            }

            pub fn name(value: $name) ->&'static str {
                match value {
                    $($name::$value => stringify!($value)),*
                }
            }
        }
    };
}

enumerable_enum!(GsdlBuiltinScalar {
    Boolean,
    Float,
    ID,
    Int,
    String,
});

#[derive(Clone, Debug, PartialEq)]
pub enum InnerTypeKind {
    Scalar,
    Vector { nullable: bool },
}
