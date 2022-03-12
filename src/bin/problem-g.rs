/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача G: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/G
 */

use lib::read_line;

struct Calculator {}
impl Calculator {
    pub fn new() -> Self {
        return Self {};
    }
    pub fn add(name: &String) -> String {}
}

fn main() {
    let n = read_line::<usize>();
    assert!(1 <= n);
    assert!(n <= 10);

    let mut calc = Calculator::new();
    for _ in 0..n {
        let name = read_line::<String>();
        assert!(name.len() <= 10);
        println!("{}", name);
    }
}

#[cfg(test)]
mod problem_d {
    #[test]
    fn sanity() {
        let mut calc = super::Calculator::new(3);
        calc.add(1);
        assert_eq!(None, calc.get(1));
        calc.add(3);
        assert_eq!(Some(2), calc.get(3));
    }
}
