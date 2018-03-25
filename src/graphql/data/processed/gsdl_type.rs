use super::field::Fields;
use super::interface::Interfaces;
use super::super::unprocessed;
use graphql::scheme::GsdlDataMap;
use std::slice::Iter;

#[derive(Debug)]
pub struct Type<'a> {
    pub name: &'a String,
    implements_iter: Iter<'a, String>,
    fields_iter: Iter<'a, unprocessed::Field>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Type<'a> {
    pub fn from(name: &'a str, data_map: &'a GsdlDataMap) -> Type<'a> {
        let gsdl_type = data_map
            .get(name)
            .expect(&format!("Type {} not found in internal data map", name));
        match *gsdl_type {
            unprocessed::GsdlDataItem::Type(ref gsdl_type) => {
                assert_eq!(*name, gsdl_type.name);
                Type {
                    name: &gsdl_type.name,
                    implements_iter: gsdl_type.implements.iter(),
                    fields_iter: gsdl_type.fields.iter(),
                    data_map,
                }
            }
            _ => panic!(
                "Expected {} to be type, but found {:?} instead",
                name, gsdl_type
            ),
        }
    }

    pub fn implements(&self) -> Interfaces {
        Interfaces::from(self.implements_iter.clone(), self.data_map)
    }

    pub fn fields(&self) -> Fields {
        Fields::from(self.fields_iter.clone(), self.data_map)
    }
}

pub struct TypeIter<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> TypeIter<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> TypeIter<'a> {
        TypeIter { iter, data_map }
    }
}

impl<'a> Iterator for TypeIter<'a> {
    type Item = Type<'a>;

    fn next(&mut self) -> Option<Type<'a>> {
        self.iter.next().map(|t| Type::from(t, self.data_map))
    }
}

pub struct Types<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Types<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> Types<'a> {
        Types { iter, data_map }
    }
}

impl<'a> IntoIterator for Types<'a> {
    type Item = Type<'a>;
    type IntoIter = TypeIter<'a>;

    fn into_iter(self) -> TypeIter<'a> {
        TypeIter::from(self.iter, self.data_map)
    }
}
