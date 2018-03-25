use super::GsdlDataItems;
use super::super::unprocessed;
use graphql::scheme::GsdlDataMap;
use std::slice::Iter;

#[derive(Debug)]
pub struct Union<'a> {
    pub name: &'a String,
    members_iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Union<'a> {
    pub fn from(name: &'a str, data_map: &'a GsdlDataMap) -> Union<'a> {
        let union = data_map
            .get(name)
            .expect(&format!("Union {} not found in internal data map", name));
        match *union {
            unprocessed::GsdlDataItem::Union(ref union) => {
                assert_eq!(*name, union.name);
                Union {
                    name: &union.name,
                    members_iter: union.members.iter(),
                    data_map,
                }
            }
            _ => panic!(
                "Expected {} to be union, but found {:?} instead",
                name, union
            ),
        }
    }

    pub fn members(&self) -> GsdlDataItems {
        GsdlDataItems::from(self.members_iter.clone(), self.data_map)
    }
}

pub struct UnionIter<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> UnionIter<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> UnionIter<'a> {
        UnionIter { iter, data_map }
    }
}

impl<'a> Iterator for UnionIter<'a> {
    type Item = Union<'a>;

    fn next(&mut self) -> Option<Union<'a>> {
        self.iter.next().map(|u| Union::from(u, self.data_map))
    }
}

pub struct Unions<'a> {
    iter: Iter<'a, String>,
    data_map: &'a GsdlDataMap,
}

impl<'a> Unions<'a> {
    pub fn from(iter: Iter<'a, String>, data_map: &'a GsdlDataMap) -> Unions<'a> {
        Unions { iter, data_map }
    }
}

impl<'a> IntoIterator for Unions<'a> {
    type Item = Union<'a>;
    type IntoIter = UnionIter<'a>;

    fn into_iter(self) -> UnionIter<'a> {
        UnionIter::from(self.iter, self.data_map)
    }
}
