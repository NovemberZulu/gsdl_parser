use std::slice::Iter;

pub struct ValueIter<'a> {
    iter: Iter<'a, String>,
}

impl<'a> ValueIter<'a> {
    pub fn from(iter: Iter<'a, String>) -> ValueIter {
        ValueIter { iter }
    }
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<&'a String> {
        self.iter.next()
    }
}

pub struct Values<'a> {
    iter: Iter<'a, String>,
}

impl<'a> Values<'a> {
    pub fn from(iter: Iter<'a, String>) -> Values {
        Values { iter }
    }
}

impl<'a> IntoIterator for Values<'a> {
    type Item = &'a String;
    type IntoIter = ValueIter<'a>;

    fn into_iter(self) -> ValueIter<'a> {
        ValueIter::from(self.iter)
    }
}
