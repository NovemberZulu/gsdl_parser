use super::argument::Arguments;
use super::super::unprocessed;
use graphql::scheme::GsdlDataMap;
use std::slice::Iter;

pub struct Field<'a> {
    pub name: &'a String,
    arguments_iter: Iter<'a, unprocessed::Argument>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Field<'a> {
    pub fn from(source: &'a unprocessed::Field, data_map: &'a GsdlDataMap) -> Field<'a> {
        Field {
            name: &source.name,
            arguments_iter: source.arguments.iter(),
            data_map,
        }
    }

    pub fn arguments(&self) -> Arguments {
        Arguments::from(self.arguments_iter.clone(), self.data_map)
    }
}

pub struct FieldIter<'a> {
    iter: Iter<'a, unprocessed::Field>,
    data_map: &'a GsdlDataMap,
}

impl<'a> FieldIter<'a> {
    pub fn from(iter: Iter<'a, unprocessed::Field>, data_map: &'a GsdlDataMap) -> FieldIter<'a> {
        FieldIter { iter, data_map }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Field<'a>;

    fn next(&mut self) -> Option<Field<'a>> {
        self.iter.next().map(|e| Field::from(e, self.data_map))
    }
}

pub struct Fields<'a> {
    iter: Iter<'a, unprocessed::Field>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Fields<'a> {
    pub fn from(iter: Iter<'a, unprocessed::Field>, data_map: &'a GsdlDataMap) -> Fields<'a> {
        Fields { iter, data_map }
    }
}

impl<'a> IntoIterator for Fields<'a> {
    type Item = Field<'a>;
    type IntoIter = FieldIter<'a>;

    fn into_iter(self) -> FieldIter<'a> {
        FieldIter::from(self.iter, self.data_map)
    }
}
