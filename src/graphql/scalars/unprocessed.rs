// scalars used by .lalrpop file
use std::vec::Vec;
pub use super::common::{GsdlBuiltinScalar, InnerTypeKind};

#[derive(Debug)]
pub enum GsdlScalar {
    Builtin(GsdlBuiltinScalar),
    Enum(Enum),
    Interface(Interface),
    Type(Type),
    Union(Union),
}

impl GsdlScalar {
    pub fn name(&self) -> &str {
        match *self {
            GsdlScalar::Builtin(builtin_scalar) => GsdlBuiltinScalar::name(builtin_scalar),
            GsdlScalar::Enum(ref gsdl_enum) => &gsdl_enum.name,
            GsdlScalar::Interface(ref interface) => &interface.name,
            GsdlScalar::Type(ref gsdl_type) => &gsdl_type.name,
            GsdlScalar::Union(ref union) => &union.name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InnerType {
    pub name: String,
    pub kind: InnerTypeKind,
}

#[derive(Debug, PartialEq)]
pub struct OuterType {
    pub nullable: bool,
    pub inner: InnerType,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

impl Enum {
    pub fn new(name: &str, values: Vec<&str>) -> Enum {
        Enum {
            name: String::from(name),
            values: values.into_iter().map(String::from).collect(),
        }
    }
}

impl Value {
    pub fn new(value: &str) -> Value {
        Value::String(String::from(value))
    }
}

// FIXME proper value support
#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub name: String,
    pub argument_type: OuterType,
    pub default: Option<Value>,
}

impl Argument {
    pub fn new(name: &str, argument_type: OuterType, default: Option<Value>) -> Argument {
        Argument {
            name: String::from(name),
            argument_type,
            default,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: OuterType,
    pub arguments: Vec<Argument>,
}

impl Field {
    pub fn new(name: &str, field_type: OuterType, arguments: Option<Vec<Argument>>) -> Field {
        Field {
            name: String::from(name),
            field_type,
            arguments: arguments.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Interface {
    pub fn new(name: &str, fields: Vec<Field>) -> Interface {
        Interface {
            name: String::from(name),
            fields,
        }
    }
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub implements: Vec<String>,
    pub fields: Vec<Field>,
}

impl Type {
    pub fn new(name: &str, implements: Option<Vec<&str>>, fields: Vec<Field>) -> Type {
        Type {
            name: String::from(name),
            implements: implements
                .unwrap_or_default()
                .into_iter()
                .map(String::from)
                .collect(),
            fields,
        }
    }
}

#[derive(Debug)]
pub struct Union {
    pub name: String,
    pub members: Vec<String>,
}

impl Union {
    pub fn new(name: &str, members_head: &str, members_tail: Vec<&str>) -> Union {
        let mut members = Vec::<String>::with_capacity(members_tail.len() + 1);
        members.push(String::from(members_head));
        members.extend(members_tail.into_iter().map(String::from));
        Union {
            name: String::from(name),
            members,
        }
    }
}

#[derive(Debug)]
pub struct SchemeEntryPoints {
    pub entries: Vec<(String, String)>,
}

impl SchemeEntryPoints {
    pub fn new(items: Vec<(&str, &str)>) -> SchemeEntryPoints {
        SchemeEntryPoints {
            entries: items
                .into_iter()
                .map(|(s1, s2)| (String::from(s1), String::from(s2)))
                .collect(),
        }
    }
}

// used in .lalrpop
pub enum GsdlItem {
    Interface(Interface),
    Type(Type),
    Enum(Enum),
    Union(Union),
    SchemeEntryPoints(SchemeEntryPoints),
}
