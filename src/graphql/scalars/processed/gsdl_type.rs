use graphql::scheme::GsdlScalarMap;
use std::slice::Iter;
use super::field::Fields;
use super::interface::Interfaces;
use super::super::unprocessed;

#[derive(Debug)]
pub struct Type<'a> {
    pub name: &'a String,
    implements_iter: Iter<'a, String>,
    fields_iter: Iter<'a, unprocessed::Field>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Type<'a> {
    pub fn from(name: &'a str, scalar_map: &'a GsdlScalarMap) -> Type<'a> {
        let gsdl_type = scalar_map
            .get(name)
            .expect(&format!("Type {} not found in internal scalar map", name));
        match *gsdl_type {
            unprocessed::GsdlScalar::Type(ref gsdl_type) => {
                assert_eq!(*name, gsdl_type.name);
                Type {
                    name: &gsdl_type.name,
                    implements_iter: gsdl_type.implements.iter(),
                    fields_iter: gsdl_type.fields.iter(),
                    scalar_map,
                }
            }
            _ => panic!(
                "Expected {} to be type, but found {:?} instead",
                name, gsdl_type
            ),
        }
    }

    pub fn implements(&self) -> Interfaces {
        Interfaces::from(self.implements_iter.clone(), self.scalar_map)
    }

    pub fn fields(&self) -> Fields {
        Fields::from(self.fields_iter.clone(), self.scalar_map)
    }
}

pub struct TypeIter<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> TypeIter<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> TypeIter<'a> {
        TypeIter { iter, scalar_map }
    }
}

impl<'a> Iterator for TypeIter<'a> {
    type Item = Type<'a>;

    fn next(&mut self) -> Option<Type<'a>> {
        self.iter.next().map(|t| Type::from(t, self.scalar_map))
    }
}

pub struct Types<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Types<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> Types<'a> {
        Types { iter, scalar_map }
    }
}

impl<'a> IntoIterator for Types<'a> {
    type Item = Type<'a>;
    type IntoIter = TypeIter<'a>;

    fn into_iter(self) -> TypeIter<'a> {
        TypeIter::from(self.iter, self.scalar_map)
    }
}
