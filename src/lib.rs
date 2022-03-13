use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;

pub fn read_line<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut line = String::new();
    let _ = std::io::stdin()
        .read_line(&mut line)
        .expect("Can't read from input stream");
    return line.trim().parse::<T>().expect("Can't parse string");
}

pub fn read_pair<F: FromStr, S: FromStr>() -> (F, S)
where
    <F as FromStr>::Err: Debug,
    <S as FromStr>::Err: Debug,
{
    let mut line = String::new();
    let _ = std::io::stdin()
        .read_line(&mut line)
        .expect("Can't read from input stream");
    let tokens: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(2, tokens.len());
    let first = tokens[0].trim().parse::<F>().expect("Can't parse string");
    let second = tokens[1].trim().parse::<S>().expect("Can't parse string");
    return (first, second);
}

#[derive(Debug)]
struct SuffixIndex {
    suffix: String,
    index: usize,
}

#[derive(Debug)]
pub struct SuffixArray {
    pub text: String,
    suffix_array: Vec<SuffixIndex>,
}

impl SuffixArray {
    pub fn get_text(&self) -> String {
        return self.text.clone();
    }

    pub fn get_suffix_idx(&self) -> Vec<usize> {
        self.suffix_array.iter().map(|x| x.index).collect()
    }

    pub fn get_suffix(&self, idx: usize) -> String {
        self.suffix_array[idx].suffix.clone()
    }

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

    pub fn get_substrs(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        for pair in self.suffix_array.iter() {
            let suffix = &pair.suffix;
            for end in 1..suffix.len() {
                set.insert(suffix[0..end].to_string());
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
        assert_eq!(vec![6, 4, 2, 1, 5, 3], sa.get_suffix_idx());
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
        assert_eq!(vec![11, 8, 1, 4, 6, 9, 2, 5, 7, 10, 3], sa.get_suffix_idx());
        assert!(sa.get_substrs().iter().all(|s| sa.search(s).is_some()));
    }
}
