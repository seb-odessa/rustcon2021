/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача C: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/C
 *
 * Целое положительное число число называется простым, если имеет ровно два различных делителя:
 * единицу и само это число.
 * Например, 2 и 3 — простые числа.
 * Число 1 имеет только один делитель, число 6 имеет четыре делителя, а число 4 имеет три делителя.
 * Поэтому 1, 6 и 4 — не простые числа.
 *
 * Требуется представить данное целое число в виде суммы двух простых.
 *
 * Входные данные
 * В единственной строке записано целое число n (1 ≤ n ≤ 100).
 *
 * Выходные данные
 * Выведите пару простых чисел, сумма которых равна n.
 * Если такой пары не существует, требуется выдать «-1».
 * Если существует несколько таких пар, разрешается выдать любую.
 */

use lib::read_line;

fn is_prime(n: u32) -> bool {
    // https://en.wikipedia.org/wiki/Primality_test
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    return true;
}

fn solve_problem(n: u32) -> Option<(u32, u32)> {
    if n <= 3 {
        return None;
    } else {
        for a in 2..(n / 2 + 1) {
            let b = n - a;
            if is_prime(a) && is_prime(b) {
                return Some((a, b));
            }
        }
    }
    return None;
}

fn main() {
    let n = read_line::<u32>();
    assert!(1 <= n);
    assert!(n <= 100);
    if let Some((a, b)) = solve_problem(n) {
        println!("{} {}", a, b);
    } else {
        println!("-1");
    }
}

#[cfg(test)]
mod problem_c {
    #[test]
    fn sanity() {
        assert_eq!(None, crate::solve_problem(1));
        assert_eq!(None, crate::solve_problem(2));
        assert_eq!(None, crate::solve_problem(3));
        assert_eq!(Some((2, 2)), crate::solve_problem(4));
        assert_eq!(Some((2, 3)), crate::solve_problem(5));
        assert_eq!(Some((3, 3)), crate::solve_problem(6));
        assert_eq!(None, crate::solve_problem(11));
    }
}
