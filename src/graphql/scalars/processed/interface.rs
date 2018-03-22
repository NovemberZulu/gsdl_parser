use graphql::scheme::GsdlScalarMap;
use std::slice::Iter;
use super::field::Fields;
use super::super::unprocessed;

#[derive(Debug)]
pub struct Interface<'a> {
    pub name: &'a String,
    fields_iter: Iter<'a, unprocessed::Field>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Interface<'a> {
    pub fn from(name: &'a str, scalar_map: &'a GsdlScalarMap) -> Interface<'a> {
        let interface = scalar_map.get(name).expect(&format!(
            "Interface {} not found in internal scalar map",
            name
        ));
        match *interface {
            unprocessed::GsdlScalar::Interface(ref interface) => {
                assert_eq!(*name, interface.name);
                Interface {
                    name: &interface.name,
                    fields_iter: interface.fields.iter(),
                    scalar_map,
                }
            }
            _ => panic!(
                "Expected {} to be interface, but found {:?} instead",
                name, interface
            ),
        }
    }

    pub fn fields(&self) -> Fields {
        Fields::from(self.fields_iter.clone(), self.scalar_map)
    }
}

pub struct InterfaceIter<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> InterfaceIter<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> InterfaceIter<'a> {
        InterfaceIter { iter, scalar_map }
    }
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = Interface<'a>;

    fn next(&mut self) -> Option<Interface<'a>> {
        self.iter
            .next()
            .map(|i| Interface::from(i, self.scalar_map))
    }
}

pub struct Interfaces<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Interfaces<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> Interfaces<'a> {
        Interfaces { iter, scalar_map }
    }
}

impl<'a> IntoIterator for Interfaces<'a> {
    type Item = Interface<'a>;
    type IntoIter = InterfaceIter<'a>;

    fn into_iter(self) -> InterfaceIter<'a> {
        InterfaceIter::from(self.iter, self.scalar_map)
    }
}
