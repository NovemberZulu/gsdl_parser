use graphql::scheme::GsdlScalarMap;
use std::slice::Iter;
use super::super::processed::OuterType;
use super::super::unprocessed;

pub struct Argument<'a> {
    pub name: &'a String,
    pub argument_type: OuterType<'a>,
    //FIXME add default value support
}

impl<'a> Argument<'a> {
    pub fn from(source: &'a unprocessed::Argument, scalar_map: &'a GsdlScalarMap) -> Argument<'a> {
        Argument {
            name: &source.name,
            argument_type: OuterType::from(&source.argument_type, scalar_map),
        }
    }
}

pub struct ArgumentIter<'a> {
    iter: Iter<'a, unprocessed::Argument>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> ArgumentIter<'a> {
    pub fn from(
        iter: Iter<'a, unprocessed::Argument>,
        scalar_map: &'a GsdlScalarMap,
    ) -> ArgumentIter<'a> {
        ArgumentIter { iter, scalar_map }
    }
}

impl<'a> Iterator for ArgumentIter<'a> {
    type Item = Argument<'a>;

    fn next(&mut self) -> Option<Argument<'a>> {
        self.iter.next().map(|a| Argument::from(a, self.scalar_map))
    }
}

pub struct Arguments<'a> {
    iter: Iter<'a, unprocessed::Argument>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Arguments<'a> {
    pub fn from(
        iter: Iter<'a, unprocessed::Argument>,
        scalar_map: &'a GsdlScalarMap,
    ) -> Arguments<'a> {
        Arguments { iter, scalar_map }
    }
}

impl<'a> IntoIterator for Arguments<'a> {
    type Item = Argument<'a>;
    type IntoIter = ArgumentIter<'a>;

    fn into_iter(self) -> ArgumentIter<'a> {
        ArgumentIter::from(self.iter, self.scalar_map)
    }
}
