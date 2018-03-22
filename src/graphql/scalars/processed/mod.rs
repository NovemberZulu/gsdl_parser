// types used in external API

use graphql::scheme::GsdlScalarMap;
pub use self::argument::Argument;
use self::gsdl_enum::Enum;
pub use self::gsdl_enum::Enums;
pub use self::gsdl_type::Type;
pub use self::gsdl_type::Types;
use self::interface::Interface;
pub use self::interface::Interfaces;
use self::union::Union;
pub use self::union::Unions;
use std::slice::Iter;
pub use super::common::{GsdlBuiltinScalar, InnerTypeKind};
use super::unprocessed;

mod argument;
mod field;
mod gsdl_enum;
mod gsdl_type;
mod interface;
mod value;
mod union;

#[derive(Debug)]
pub enum GsdlScalar<'a> {
    Builtin(GsdlBuiltinScalar),
    Enum(Enum<'a>),
    Interface(Interface<'a>),
    Type(Type<'a>),
    Union(Union<'a>),
}

impl<'a> GsdlScalar<'a> {
    pub fn from(name: &'a str, scalar_map: &'a GsdlScalarMap) -> GsdlScalar<'a> {
        let source = scalar_map
            .get(name)
            .expect(&format!("Scalar {} not found in internal scalar map", name));
        match *source {
            unprocessed::GsdlScalar::Builtin(builtin_scalar) => {
                assert_eq!(*name, *GsdlBuiltinScalar::name(builtin_scalar));
                GsdlScalar::Builtin(builtin_scalar)
            }
            unprocessed::GsdlScalar::Enum(ref gsdl_enum) => {
                assert_eq!(*name, gsdl_enum.name);
                GsdlScalar::Enum(Enum::from(&gsdl_enum.name, scalar_map))
            }
            unprocessed::GsdlScalar::Interface(ref interface) => {
                assert_eq!(*name, interface.name);
                GsdlScalar::Interface(Interface::from(&interface.name, scalar_map))
            }
            unprocessed::GsdlScalar::Type(ref gsdl_type) => {
                assert_eq!(*name, gsdl_type.name);
                GsdlScalar::Type(Type::from(&gsdl_type.name, scalar_map))
            }
            unprocessed::GsdlScalar::Union(ref union) => {
                assert_eq!(*name, union.name);
                GsdlScalar::Union(Union::from(&union.name, scalar_map))
            }
        }
    }

    pub fn name(&self) -> &str {
        match *self {
            GsdlScalar::Builtin(builtin_scalar) => GsdlBuiltinScalar::name(builtin_scalar),
            GsdlScalar::Enum(ref gsdl_enum) => gsdl_enum.name,
            GsdlScalar::Interface(ref interface) => interface.name,
            GsdlScalar::Type(ref gsdl_type) => gsdl_type.name,
            GsdlScalar::Union(ref union) => union.name,
        }
    }
}

pub struct GsdlScalarIter<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> GsdlScalarIter<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> GsdlScalarIter<'a> {
        GsdlScalarIter { iter, scalar_map }
    }
}

impl<'a> Iterator for GsdlScalarIter<'a> {
    type Item = GsdlScalar<'a>;

    fn next(&mut self) -> Option<GsdlScalar<'a>> {
        self.iter
            .next()
            .map(|s| GsdlScalar::from(s, self.scalar_map))
    }
}

pub struct GsdlScalars<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> GsdlScalars<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> GsdlScalars<'a> {
        GsdlScalars { iter, scalar_map }
    }
}

impl<'a> IntoIterator for GsdlScalars<'a> {
    type Item = GsdlScalar<'a>;
    type IntoIter = GsdlScalarIter<'a>;

    fn into_iter(self) -> GsdlScalarIter<'a> {
        GsdlScalarIter::from(self.iter, self.scalar_map)
    }
}

pub struct InnerType<'a> {
    pub scalar: GsdlScalar<'a>,
    pub kind: InnerTypeKind,
}

impl<'a> InnerType<'a> {
    pub fn from(
        source: &'a unprocessed::InnerType,
        scalar_map: &'a GsdlScalarMap,
    ) -> InnerType<'a> {
        InnerType {
            scalar: GsdlScalar::from(&source.name, scalar_map),
            kind: source.kind.clone(),
        }
    }
}

pub struct OuterType<'a> {
    pub nullable: bool,
    pub inner: InnerType<'a>,
}

impl<'a> OuterType<'a> {
    pub fn from(
        source: &'a unprocessed::OuterType,
        scalar_map: &'a GsdlScalarMap,
    ) -> OuterType<'a> {
        OuterType {
            nullable: source.nullable,
            inner: InnerType::from(&source.inner, scalar_map),
        }
    }
}
