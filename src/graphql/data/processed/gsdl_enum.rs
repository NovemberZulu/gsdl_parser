use super::super::unprocessed;
use super::value::Values;
use graphql::scheme::GsdlDataMap;
use std::slice::Iter;

#[derive(Debug)]
pub struct Enum<'a> {
    pub name: &'a String,
    values_iter: Iter<'a, String>,
}

impl<'a> Enum<'a> {
    pub fn from(name: &'a str, data_map: &'a GsdlDataMap) -> Enum<'a> {
        let gsdl_enum = data_map
            .get(name)
            .expect(&format!("Enum {} not found in internal data map", name));
        match *gsdl_enum {
            unprocessed::GsdlDataItem::Enum(ref gsdl_enum) => {
                assert_eq!(*name, gsdl_enum.name);
                Enum {
                    name: &gsdl_enum.name,
                    values_iter: gsdl_enum.values.iter(),
                }
            }
            _ => panic!(
                "Expected {} to be enum, but found {:?} instead",
                name, gsdl_enum
            ),
        }
    }

    pub fn values(&self) -> Values {
        Values::from(self.values_iter.clone())
    }
}

pub struct EnumIter<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> EnumIter<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> EnumIter<'a> {
        EnumIter { iter, data_map }
    }
}

impl<'a> Iterator for EnumIter<'a> {
    type Item = Enum<'a>;

    fn next(&mut self) -> Option<Enum<'a>> {
        self.iter.next().map(|e| Enum::from(e, self.data_map))
    }
}

pub struct Enums<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Enums<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> Enums<'a> {
        Enums { iter, data_map }
    }
}

impl<'a> IntoIterator for Enums<'a> {
    type Item = Enum<'a>;
    type IntoIter = EnumIter<'a>;

    fn into_iter(self) -> EnumIter<'a> {
        EnumIter::from(self.iter, self.data_map)
    }
}
