use super::super::processed::OuterType;
use super::super::unprocessed;
use graphql::scheme::GsdlDataMap;
use std::slice::Iter;

pub struct Argument<'a> {
    pub name: &'a String,
    pub argument_type: OuterType<'a>,
    //FIXME add default value support
}

impl<'a> Argument<'a> {
    pub fn from(source: &'a unprocessed::Argument, data_map: &'a GsdlDataMap) -> Argument<'a> {
        Argument {
            name: &source.name,
            argument_type: OuterType::from(&source.argument_type, data_map),
        }
    }
}

pub struct ArgumentIter<'a> {
    iter: Iter<'a, unprocessed::Argument>,
    data_map: &'a GsdlDataMap,
}

impl<'a> ArgumentIter<'a> {
    pub fn from(
        iter: Iter<'a, unprocessed::Argument>,
        data_map: &'a GsdlDataMap,
    ) -> ArgumentIter<'a> {
        ArgumentIter { iter, data_map }
    }
}

impl<'a> Iterator for ArgumentIter<'a> {
    type Item = Argument<'a>;

    fn next(&mut self) -> Option<Argument<'a>> {
        self.iter.next().map(|a| Argument::from(a, self.data_map))
    }
}

pub struct Arguments<'a> {
    iter: Iter<'a, unprocessed::Argument>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Arguments<'a> {
    pub fn from(iter: Iter<'a, unprocessed::Argument>, data_map: &'a GsdlDataMap) -> Arguments<'a> {
        Arguments { iter, data_map }
    }
}

impl<'a> IntoIterator for Arguments<'a> {
    type Item = Argument<'a>;
    type IntoIter = ArgumentIter<'a>;

    fn into_iter(self) -> ArgumentIter<'a> {
        ArgumentIter::from(self.iter, self.data_map)
    }
}
