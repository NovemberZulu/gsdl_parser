// types used in external API

pub use self::argument::Argument;
use self::gsdl_enum::Enum;
pub use self::gsdl_enum::Enums;
pub use self::gsdl_type::Type;
pub use self::gsdl_type::Types;
use self::interface::Interface;
pub use self::interface::Interfaces;
use self::union::Union;
pub use self::union::Unions;
pub use super::common::{InnerTypeKind, Scalar};
use super::unprocessed;
use graphql::scheme::GsdlDataMap;
use std::slice::Iter;

mod argument;
mod field;
mod gsdl_enum;
mod gsdl_type;
mod interface;
mod union;
mod value;

#[derive(Debug)]
pub enum GsdlDataItem<'a> {
    Builtin(Scalar),
    Enum(Enum<'a>),
    Interface(Interface<'a>),
    Type(Type<'a>),
    Union(Union<'a>),
}

impl<'a> GsdlDataItem<'a> {
    pub fn from(name: &'a str, data_map: &'a GsdlDataMap) -> GsdlDataItem<'a> {
        let source = data_map
            .get(name)
            .expect(&format!("Item {} not found in internal data map", name));
        match *source {
            unprocessed::GsdlDataItem::Builtin(scalar) => {
                assert_eq!(*name, *Scalar::name(scalar));
                GsdlDataItem::Builtin(scalar)
            }
            unprocessed::GsdlDataItem::Enum(ref gsdl_enum) => {
                assert_eq!(*name, gsdl_enum.name);
                GsdlDataItem::Enum(Enum::from(&gsdl_enum.name, data_map))
            }
            unprocessed::GsdlDataItem::Interface(ref interface) => {
                assert_eq!(*name, interface.name);
                GsdlDataItem::Interface(Interface::from(&interface.name, data_map))
            }
            unprocessed::GsdlDataItem::Type(ref gsdl_type) => {
                assert_eq!(*name, gsdl_type.name);
                GsdlDataItem::Type(Type::from(&gsdl_type.name, data_map))
            }
            unprocessed::GsdlDataItem::Union(ref union) => {
                assert_eq!(*name, union.name);
                GsdlDataItem::Union(Union::from(&union.name, data_map))
            }
        }
    }

    pub fn name(&self) -> &str {
        match *self {
            GsdlDataItem::Builtin(scalar) => Scalar::name(scalar),
            GsdlDataItem::Enum(ref gsdl_enum) => gsdl_enum.name,
            GsdlDataItem::Interface(ref interface) => interface.name,
            GsdlDataItem::Type(ref gsdl_type) => gsdl_type.name,
            GsdlDataItem::Union(ref union) => union.name,
        }
    }
}

pub struct GsdlDataItemIter<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> GsdlDataItemIter<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> GsdlDataItemIter<'a> {
        GsdlDataItemIter { iter, data_map }
    }
}

impl<'a> Iterator for GsdlDataItemIter<'a> {
    type Item = GsdlDataItem<'a>;

    fn next(&mut self) -> Option<GsdlDataItem<'a>> {
        self.iter
            .next()
            .map(|s| GsdlDataItem::from(s, self.data_map))
    }
}

pub struct GsdlDataItems<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> GsdlDataItems<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> GsdlDataItems<'a> {
        GsdlDataItems { iter, data_map }
    }
}

impl<'a> IntoIterator for GsdlDataItems<'a> {
    type Item = GsdlDataItem<'a>;
    type IntoIter = GsdlDataItemIter<'a>;

    fn into_iter(self) -> GsdlDataItemIter<'a> {
        GsdlDataItemIter::from(self.iter, self.data_map)
    }
}

pub struct InnerType<'a> {
    pub scalar: GsdlDataItem<'a>,
    pub kind: InnerTypeKind,
}

impl<'a> InnerType<'a> {
    pub fn from(source: &'a unprocessed::InnerType, data_map: &'a GsdlDataMap) -> InnerType<'a> {
        InnerType {
            scalar: GsdlDataItem::from(&source.name, data_map),
            kind: source.kind.clone(),
        }
    }
}

pub struct OuterType<'a> {
    pub nullable: bool,
    pub inner: InnerType<'a>,
}

impl<'a> OuterType<'a> {
    pub fn from(source: &'a unprocessed::OuterType, data_map: &'a GsdlDataMap) -> OuterType<'a> {
        OuterType {
            nullable: source.nullable,
            inner: InnerType::from(&source.inner, data_map),
        }
    }
}
