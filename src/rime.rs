use std::cmp::{self, Ordering};

pub mod yaml;

#[derive(Debug)]
pub struct Dict {
    chars: Vec<DictEntry>,
    phrases: Vec<DictEntry>,
}

impl Dict {
    pub(crate) fn new(chars: Vec<DictEntry>, phrases: Vec<DictEntry>) -> Self {
        debug_assert!(chars.iter().all(|entry| entry.phrase.chars().count() == 1));
        debug_assert!(phrases.iter().all(|entry| entry.phrase.chars().count() > 1));
        debug_assert!(chars.is_sorted() && phrases.is_sorted());
        Self { chars, phrases }
    }

    pub fn contains_chars(&self) -> bool {
        !self.chars.is_empty()
    }
    pub fn contains_phrases(&self) -> bool {
        !self.phrases.is_empty()
    }
    pub fn contains_both(&self) -> bool {
        self.contains_chars() && self.contains_phrases()
    }

    pub fn chars(&self) -> impl Iterator<Item = &DictEntry> {
        self.chars.iter()
    }

    pub fn phrases(&self) -> impl Iterator<Item = &DictEntry> {
        self.phrases.iter()
    }

    pub fn entries(&self) -> impl Iterator<Item = &DictEntry> {
        self.chars().chain(self.phrases())
    }

    pub fn into_raw_parts(self) -> (Vec<DictEntry>, Vec<DictEntry>) {
        (self.chars, self.phrases)
    }
}

fn insert<T: cmp::PartialOrd>(vec: &mut Vec<T>, element: T) {
    if let Some(max) = vec.last() {
        if *max <= element {
            vec.push(element);
        } else {
            // This is unexpected
            let index = vec.partition_point(|e| *e <= element);
            vec.insert(index, element);
        }
    } else {
        vec.push(element);
    }
}

impl FromIterator<DictEntry> for Dict {
    fn from_iter<T: IntoIterator<Item = DictEntry>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let capacity = cmp::min(upper.unwrap_or(lower), isize::MAX as usize / 2);
        let mut chars = Vec::with_capacity(capacity);
        let mut phrases = Vec::with_capacity(capacity);
        for entry in iter {
            if entry.phrase.chars().count() == 1 {
                insert(&mut chars, entry);
            } else {
                insert(&mut phrases, entry);
            }
        }
        Self::new(chars, phrases)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictEntry {
    pub(crate) code: String,
    pub(crate) weight: Option<u32>,
    pub(crate) phrase: String,
}

impl Ord for DictEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code
            .cmp(&other.code)
            .then(other.weight.cmp(&self.weight))
            .then(self.phrase.cmp(&other.phrase))
    }
}

impl PartialOrd for DictEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
