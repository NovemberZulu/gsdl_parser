use graphql::scheme::GsdlScalarMap;
use std::slice::Iter;
use super::argument::Arguments;
use super::super::unprocessed;

pub struct Field<'a> {
    pub name: &'a String,
    arguments_iter: Iter<'a, unprocessed::Argument>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Field<'a> {
    pub fn from(source: &'a unprocessed::Field, scalar_map: &'a GsdlScalarMap) -> Field<'a> {
        Field {
            name: &source.name,
            arguments_iter: source.arguments.iter(),
            scalar_map,
        }
    }

    pub fn arguments(&self) -> Arguments {
        Arguments::from(self.arguments_iter.clone(), self.scalar_map)
    }
}

pub struct FieldIter<'a> {
    iter: Iter<'a, unprocessed::Field>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> FieldIter<'a> {
    pub fn from(
        iter: Iter<'a, unprocessed::Field>,
        scalar_map: &'a GsdlScalarMap,
    ) -> FieldIter<'a> {
        FieldIter { iter, scalar_map }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Field<'a>;

    fn next(&mut self) -> Option<Field<'a>> {
        self.iter.next().map(|e| Field::from(e, self.scalar_map))
    }
}

pub struct Fields<'a> {
    iter: Iter<'a, unprocessed::Field>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Fields<'a> {
    pub fn from(iter: Iter<'a, unprocessed::Field>, scalar_map: &'a GsdlScalarMap) -> Fields<'a> {
        Fields { iter, scalar_map }
    }
}

impl<'a> IntoIterator for Fields<'a> {
    type Item = Field<'a>;
    type IntoIter = FieldIter<'a>;

    fn into_iter(self) -> FieldIter<'a> {
        FieldIter::from(self.iter, self.scalar_map)
    }
}
