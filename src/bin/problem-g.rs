/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача G: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/G
 */

use lib::read_line;
use lib::SuffixArray;
use std::collections::HashSet;

struct Calculator {
    patterns: HashSet<String>,
}
impl Calculator {
    pub fn new() -> Self {
        return Self {
            patterns: HashSet::new(),
        };
    }

    fn is_searcheable(name: &String) -> bool {
        return "+" == &name[name.len() - 1..];
    }

    pub fn add<T: Into<String>>(&mut self, name: T) -> String {
        let name = name.into();

        if Self::is_searcheable(&name) {
            let sa = SuffixArray::new(name[0..name.len() - 1].to_string());
            let subs = sa.get_substrings();
            if self.patterns.is_empty() {
                self.patterns = subs
            } else {
                self.patterns = self.patterns.intersection(&subs).cloned().collect();
            }
        } else {
            let sa = SuffixArray::new(name);
            let mut keep = HashSet::new();
            for pattern in self.patterns.iter() {
                if sa.search(pattern).is_none() {
                    keep.insert(pattern.clone());
                }
            }
            self.patterns = keep;
        }

        let mut v = self.patterns.iter().collect::<Vec<&String>>();
        v.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));
        v.into_iter()
            .next()
            .unwrap_or(&String::from("-1"))
            .to_string()
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
        println!("{}", calc.add(name));
    }
}

#[cfg(test)]
mod problem_g {
    #[test]
    fn sanity() {
        let mut calc = super::Calculator::new();
        assert_eq!("_", calc.add("mit_kotiki+"));
        assert_eq!("_", calc.add("sjtu_koty+"));
        assert_eq!("k", calc.add("itmo_first"));
        assert_eq!("ot", calc.add("msu_koshki"));
        assert_eq!("kot", calc.add("mipt_alot"));
        assert_eq!("-1", calc.add("spsu_kot"));
    }
}
