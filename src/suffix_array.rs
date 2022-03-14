use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug)]
struct SuffixIndex {
    suffix: String,
    index: usize,
}

#[derive(Debug)]
pub struct SuffixArray {
    text: String,
    suffix_array: Vec<SuffixIndex>,
}

impl SuffixArray {
    pub fn new<T: Into<String>>(text: T) -> Self {
        let text = text.into();
        let mut suffix_array = Vec::with_capacity(text.len());
        for idx in 0..(text.len()) {
            let suffix = text[idx..].to_string();
            suffix_array.push(SuffixIndex {
                suffix: suffix,
                index: idx + 1,
            });
        }
        suffix_array.sort_by(|a, b| a.suffix.cmp(&b.suffix));

        Self { text, suffix_array }
    }

    pub fn search<T: Into<String>>(&self, pattern: T) -> Option<(usize, usize)> {
        //  Return indices [s, r) such that the interval A[s..r]
        //  represents all suffixes of S that start with the pattern P.
        let p = pattern.into();
        let mut left = 0;
        let mut right = self.text.len();
        while left < right {
            let middle = (left + right) / 2;
            if p > self.suffix_array[middle].suffix {
                left = middle + 1
            } else {
                right = middle;
            }
        }
        let start = left;
        right = self.text.len();
        while left < right {
            let middle = (left + right) / 2;
            let suffix = &self.suffix_array[middle].suffix;
            if suffix.starts_with(&p) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        if start != right {
            return Some((start, right));
        } else {
            return None;
        }
    }

    pub fn get_index_array(&self) -> Vec<usize> {
        self.suffix_array.iter().map(|x| x.index).collect()
    }

    pub fn get_suffix(&self, idx: usize) -> Option<String> {
        if idx < self.suffix_array.len() {
            Some(self.suffix_array[idx].suffix.clone())
        } else {
            None
        }
    }

    pub fn get_substrings(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        for pair in self.suffix_array.iter() {
            let suffix = &pair.suffix;
            for end in 1..suffix.len() + 1 {
                let substring = suffix[0..end].to_string();
                set.insert(substring);
            }
        }
        return set;
    }
}

#[cfg(test)]
mod suffix_array {
    #[test]
    fn sanity_banana() {
        let sa = super::SuffixArray::new("banana");
        assert_eq!(vec![6, 4, 2, 1, 5, 3], sa.get_index_array());
        assert_eq!(Some(String::from("a")), sa.get_suffix(0));
        assert_eq!(Some(String::from("ana")), sa.get_suffix(1));
        assert_eq!(Some(String::from("anana")), sa.get_suffix(2));
        assert_eq!(Some(String::from("banana")), sa.get_suffix(3));
        assert_eq!(Some(String::from("na")), sa.get_suffix(4));
        assert_eq!(Some(String::from("nana")), sa.get_suffix(5));

        assert_eq!(Some((0, 3)), sa.search("a"));
        assert_eq!(Some((1, 3)), sa.search("an"));
        assert_eq!(Some((2, 3)), sa.search("anan"));

        assert_eq!(None, sa.search("ananas"));
        assert_eq!(None, sa.search("apple"));
        assert_eq!(None, sa.search("c"));
    }

    #[test]
    fn sanity_abracadabra() {
        let sa = super::SuffixArray::new("abracadabra");
        assert_eq!(
            vec![11, 8, 1, 4, 6, 9, 2, 5, 7, 10, 3],
            sa.get_index_array()
        );
        assert!(sa.get_substrings().iter().all(|s| sa.search(s).is_some()));
    }

    #[test]
    fn sanity_get_substrings() {
        let sa = super::SuffixArray::new("abcd");
        let mut vec = sa.get_substrings().into_iter().collect::<Vec<String>>();
        vec.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));
        assert_eq!(
            vec!["a", "b", "c", "d", "ab", "bc", "cd", "abc", "bcd", "abcd"],
            vec
        );
    }
}
