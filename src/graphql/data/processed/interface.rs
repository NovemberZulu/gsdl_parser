use super::field::Fields;
use super::super::unprocessed;
use graphql::scheme::GsdlDataMap;
use std::slice::Iter;

#[derive(Debug)]
pub struct Interface<'a> {
    pub name: &'a String,
    fields_iter: Iter<'a, unprocessed::Field>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Interface<'a> {
    pub fn from(name: &'a str, data_map: &'a GsdlDataMap) -> Interface<'a> {
        let interface = data_map.get(name).expect(&format!(
            "Interface {} not found in internal data map",
            name
        ));
        match *interface {
            unprocessed::GsdlDataItem::Interface(ref interface) => {
                assert_eq!(*name, interface.name);
                Interface {
                    name: &interface.name,
                    fields_iter: interface.fields.iter(),
                    data_map,
                }
            }
            _ => panic!(
                "Expected {} to be interface, but found {:?} instead",
                name, interface
            ),
        }
    }

    pub fn fields(&self) -> Fields {
        Fields::from(self.fields_iter.clone(), self.data_map)
    }
}

pub struct InterfaceIter<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> InterfaceIter<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> InterfaceIter<'a> {
        InterfaceIter { iter, data_map }
    }
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = Interface<'a>;

    fn next(&mut self) -> Option<Interface<'a>> {
        self.iter.next().map(|i| Interface::from(i, self.data_map))
    }
}

pub struct Interfaces<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Interfaces<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> Interfaces<'a> {
        Interfaces { iter, data_map }
    }
}

impl<'a> IntoIterator for Interfaces<'a> {
    type Item = Interface<'a>;
    type IntoIter = InterfaceIter<'a>;

    fn into_iter(self) -> InterfaceIter<'a> {
        InterfaceIter::from(self.iter, self.data_map)
    }
}
