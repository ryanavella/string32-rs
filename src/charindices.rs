use std::convert::TryInto;

use super::Str32;

#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct CharIndices<'a> {
    iter: std::str::CharIndices<'a>,
}

impl<'a> From<&'a Str32> for CharIndices<'a> {
    fn from(s: &'a Str32) -> Self {
        Self {
            iter: s.as_str().char_indices(),
        }
    }
}

impl<'a> Iterator for CharIndices<'a> {
    type Item = (u32, char);

    fn next(&mut self) -> Option<(u32, char)> {
        self.iter.next().map(|(i, c)| (i.try_into().unwrap(), c))
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn last(self) -> Option<(u32, char)> {
        self.iter.last().map(|(i, c)| (i.try_into().unwrap(), c))
    }
}
