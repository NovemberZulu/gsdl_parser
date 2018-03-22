use graphql::scheme::GsdlScalarMap;
use std::slice::Iter;
use super::GsdlScalars;
use super::super::unprocessed;

#[derive(Debug)]
pub struct Union<'a> {
    pub name: &'a String,
    members_iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Union<'a> {
    pub fn from(name: &'a str, scalar_map: &'a GsdlScalarMap) -> Union<'a> {
        let union = scalar_map
            .get(name)
            .expect(&format!("Union {} not found in internal scalar map", name));
        match *union {
            unprocessed::GsdlScalar::Union(ref union) => {
                assert_eq!(*name, union.name);
                Union {
                    name: &union.name,
                    members_iter: union.members.iter(),
                    scalar_map,
                }
            }
            _ => panic!(
                "Expected {} to be union, but found {:?} instead",
                name, union
            ),
        }
    }

    pub fn members(&self) -> GsdlScalars {
        GsdlScalars::from(self.members_iter.clone(), self.scalar_map)
    }
}

pub struct UnionIter<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> UnionIter<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> UnionIter<'a> {
        UnionIter { iter, scalar_map }
    }
}

impl<'a> Iterator for UnionIter<'a> {
    type Item = Union<'a>;

    fn next(&mut self) -> Option<Union<'a>> {
        self.iter.next().map(|u| Union::from(u, self.scalar_map))
    }
}

pub struct Unions<'a> {
    iter: Iter<'a, String>,
    scalar_map: &'a GsdlScalarMap,
}

impl<'a> Unions<'a> {
    pub fn from(iter: Iter<'a, String>, scalar_map: &'a GsdlScalarMap) -> Unions<'a> {
        Unions { iter, scalar_map }
    }
}

impl<'a> IntoIterator for Unions<'a> {
    type Item = Union<'a>;
    type IntoIter = UnionIter<'a>;

    fn into_iter(self) -> UnionIter<'a> {
        UnionIter::from(self.iter, self.scalar_map)
    }
}
