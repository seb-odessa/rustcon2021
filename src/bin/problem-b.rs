/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача B: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/B
 *
 * Две трубы наполняют бассейн за A часов.
 * Одна первая труба наполнила бы этот бассейн за B часов.
 * Найдите время наполнения этого бассейна одной второй трубой.
 *
 * Гарантируется, что тесты устроены так, что ответ на задачу всегда целый и не превосходит 10^6.
 *
 * Входные данные
 * В первой строке дано целое число A, во второй строке дано целое число B (1 ≤ A < B ≤ 10^6).
 *
 * Выходные данные
 * Требуется выдать единственное целое число — ответ на задачу в часах.
 */

use lib::read_line;

fn solve_problem(a: u32, b: u32) -> u32 {
    // Ax + Ay = Bx = Cy;
    // Ay = Bx - Ax; y = (Bx - Ax) / A = (B - A)x / A;
    // C = Bx / y = Bx / (B - A)x / A = BA / (B - A);
    b * a / (b - a)
}

fn main() {
    let a = read_line::<u32>();
    assert!(1 <= a);
    let b = read_line::<u32>();
    assert!(a < b);
    assert!(b <= 1000000);
    println!("{}", solve_problem(a, b));
}

#[cfg(test)]
mod problem_b {
    #[test]
    fn sanity() {
        assert_eq!(2, crate::solve_problem(1, 2));
    }
}
