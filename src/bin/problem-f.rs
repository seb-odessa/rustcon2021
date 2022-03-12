/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача F: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/F
 *
 */

use lib::read_line;
use std::collections::VecDeque;

static MAP: &[(&'static str, u8)] = &[
    ("rbgw", 2),
    ("rbwg", 3),
    ("rgbw", 5),
    ("rgwb", 1),
    ("rwbg", 4),
    ("rwgb", 6),
];

pub fn find(key: &str) -> Option<u8> {
    MAP.binary_search_by(|(k, _)| k.cmp(&key))
        .map(|x| MAP[x].1)
        .ok()
}

fn solve_problem(code: &str) -> Option<u8> {
    let mut deque = code.bytes().collect::<VecDeque<u8>>();
    for shift in 0..code.len() {
        deque.rotate_right(shift);
        let copy = deque.range(..).copied().collect::<Vec<u8>>();
        let shifted = std::str::from_utf8(&copy).ok()?;
        let result = find(shifted);
        if result.is_some() {
            return result;
        }
    }
    return None;
}

fn main() {
    let code = read_line::<String>();
    assert!(4 == code.len());
    if let Some(mode) = solve_problem(&code) {
        println!("{}", mode);
    } else {
        println!("Invalid input data: {}", code);
    }
}

#[cfg(test)]
mod problem_c {
    #[test]
    fn sanity() {
        assert_eq!(None, crate::solve_problem("aaaa"));

        assert_eq!(Some(1), crate::solve_problem("rgwb"));
        assert_eq!(Some(1), crate::solve_problem("gwbr"));
        assert_eq!(Some(1), crate::solve_problem("wbrg"));
        assert_eq!(Some(1), crate::solve_problem("brgw"));

        assert_eq!(Some(2), crate::solve_problem("rbgw"));
        assert_eq!(Some(2), crate::solve_problem("bgwr"));
        assert_eq!(Some(2), crate::solve_problem("gwrb"));
        assert_eq!(Some(2), crate::solve_problem("wrbg"));

        assert_eq!(Some(3), crate::solve_problem("rbwg"));
        assert_eq!(Some(3), crate::solve_problem("bwgr"));
        assert_eq!(Some(3), crate::solve_problem("wgrb"));
        assert_eq!(Some(3), crate::solve_problem("grbw"));

        assert_eq!(Some(4), crate::solve_problem("rwbg"));
        assert_eq!(Some(4), crate::solve_problem("wbgr"));
        assert_eq!(Some(4), crate::solve_problem("bgrw"));
        assert_eq!(Some(4), crate::solve_problem("grwb"));

        assert_eq!(Some(5), crate::solve_problem("rgbw"));
        assert_eq!(Some(5), crate::solve_problem("gbwr"));
        assert_eq!(Some(5), crate::solve_problem("bwrg"));
        assert_eq!(Some(5), crate::solve_problem("wrgb"));

        assert_eq!(Some(6), crate::solve_problem("rwgb"));
        assert_eq!(Some(6), crate::solve_problem("wgbr"));
        assert_eq!(Some(6), crate::solve_problem("gbrw"));
        assert_eq!(Some(6), crate::solve_problem("brwg"));
    }
}
