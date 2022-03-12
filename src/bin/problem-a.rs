/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача А: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/A
 *
 * Миша хочет стать очень сильным программистом. Поэтому он качает пальцы, вися на турнике.
 *
 * Миша выполняет N подходов к турнику.
 * При каждом подходе он висит на турнике T секунд.
 * Между подходами Миша отдыхает P секунд.
 * Тренировка считается оконченной после последнего подхода.
 *
 * Чтобы чётко следовать плану тренировки, Миша использует приложение на телефоне, которое
 * пищит, когда Мише нужно залезть на турник или слезть с турника.
 *
 * Однако Мише не хватает ещё одного приложения, которое сможет до начала тренировки сказать,
 * сколько секунд составит её длительность. Приложение должно принимать на вход числа N, P и T,
 * после чего выдавать общее время тренировки. Ваша задача — реализовать такое приложение.
 *
 * Входные данные
 * Входные данные содержат целые числа T, P, N (1 ≤ T, P ≤ 10;2 ≤ N ≤ 10).
 * Каждое число записано с новой строки.
 *
 * Выходные данные
 * Выведите целое число — общее время тренировки в секундах.
 */

use lib::read_line;

fn solve_problem(t: u32, p: u32, n: u32) -> u32 {
    if 0 == n {
        0
    } else if 1 == n {
        t
    } else {
        t + p + solve_problem(t, p, n - 1)
    }
}

fn main() {
    let t = read_line::<u32>();
    assert!(1 <= t);
    let p = read_line::<u32>();
    assert!(p <= 10);
    let n = read_line::<u32>();
    assert!(n <= 10);

    println!("{}", solve_problem(t, p, n));
}

#[cfg(test)]
mod probleb_a {
    #[test]
    fn sanity() {
        assert_eq!(22, crate::solve_problem(2, 3, 5));
    }
    #[test]
    fn single_workout() {
        assert_eq!(50, crate::solve_problem(50, 5, 1));
    }
    #[test]
    fn no_workout() {
        assert_eq!(0, crate::solve_problem(50, 10, 0));
    }
    #[test]
    fn no_rest_workout() {
        assert_eq!(100, crate::solve_problem(10, 0, 10));
    }
}
