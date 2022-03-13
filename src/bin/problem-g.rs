/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача G: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/G
 */

use lib::read_line;
use lib::SuffixArray;
use std::collections::HashSet;

struct Calculator {
    suffixes: Vec<SuffixArray>,
    patterns: HashSet<String>,
}
impl Calculator {
    pub fn new() -> Self {
        return Self {
            suffixes: Vec::new(),
            patterns: HashSet::new(),
        };
    }
    fn is_should_be_searched(name: String) -> bool {
        return "+" == &name[name.len() - 1..];
    }

    fn get_search_patterns(&self) -> Vec<String> {
        let mut set = HashSet::new();
        for sa in self.suffixes.iter() {
            if Self::is_should_be_searched(sa.get_text()) {
                if set.is_empty() {
                    set = sa.get_substrs();
                } else {
                    set = set.intersection(&sa.get_substrs()).cloned().collect();
                }
            }
        }
        let mut v = set.into_iter().collect::<Vec<String>>();
        v.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));
        return v;
    }

    pub fn add<T: Into<String>>(&mut self, name: T) -> String {
        self.suffixes.push(SuffixArray::new(name.into()));
        let mut sp = self.get_search_patterns();
        for sa in self.suffixes.iter() {
            if !Self::is_should_be_searched(sa.get_text()) {
                let mut keep = Vec::new();
                for pattern in sp {
                    if sa.search(&pattern).is_none() {
                        keep.push(pattern);
                    }
                }
                sp = keep;
            }
        }
        println!("sp: {:?}\n", sp);
        if sp.is_empty() {
            String::from("-1")
        } else {
            sp[0].clone()
        }
    }

    pub fn add2<T: Into<String>>(&mut self, name: T) -> String {
        let sa = SuffixArray::new(name.into());

        if Self::is_should_be_searched(sa.get_text()) {
            let subs = sa.get_substrs();
            if self.patterns.is_empty() {
                self.patterns = subs
            } else {
                self.patterns = self.patterns.intersection(&subs).cloned().collect();
            }
        } else {
            let mut keep = HashSet::new();
            for pattern in self.patterns.iter() {
                if sa.search(pattern).is_none() {
                    keep.insert(pattern.clone());
                }
            }
            self.patterns = keep;
        }

        let mut v = self.patterns.iter().cloned().collect::<Vec<String>>();
        v.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));
        println!("sp: {:?}\n", v);
        if v.is_empty() {
            String::from("-1")
        } else {
            v[0].clone()
        }
    }
}

fn main() {
    let n = read_line::<usize>();
    assert!(1 <= n);
    assert!(n <= 10);

    let mut calc = Calculator::new();
    for _ in 0..n {
        let name = read_line::<String>();
        assert!(name.len() <= 11);
        println!("{}", calc.add2(name));
    }
}

#[cfg(test)]
mod problem_d {
    #[test]
    fn sanity() {
        let mut calc = super::Calculator::new();
        assert_eq!("_", calc.add2("mit_kotiki+"));
        assert_eq!("_", calc.add2("sjtu_koty+"));
        assert_eq!("k", calc.add2("itmo_first"));
        assert_eq!("ot", calc.add2("msu_koshki"));
        assert_eq!("kot", calc.add2("mipt_alot"));
        assert_eq!("-1", calc.add2("spsu_kot"));
    }
}
