/*
 * RustCon Confest 2021 (https://codeforces.com/group/aZqKf857j1/contest/348355)
 * Задача D: https://codeforces.com/group/aZqKf857j1/contest/348355/problem/D
 *
 * Пусть у вас есть массив из n ячеек, во всех ячейках которого изначально стоят нули.
 * Вам подаются запросы двух видов:
 *
 *  add x — изменить число в ячейке x с нуля на единицу.
 *          Гарантируется, что перед вызовом этого запроса в ячейке x всегда записан нуль.
 *  get x — выдать расстояние до ближайшей ячейки к ячейке x, в которой записана единица
 *          (за исключением самой ячейки x).
 *          Гарантируется, что до вызова запроса в ячейке x всегда записана единица.
 *          Если x — единственная единица в массиве, выведите  - 1.
 *
 * Входные данные
 * В первой строке записаны целые числа n и m (1 ≤ n ≤ 5 × 10^5; 1 ≤ m ≤ 5 × 10^5).
 * Далее даны m запросов, тип которых описан в условии.
 *
 * Выходные данные
 * Для каждого запроса get нужно выдать по одному числу — ответ на запрос.
 */

use lib::read_pair;

struct Calculator {
    array: Vec<u8>,
}
impl Calculator {
    pub fn new(size: usize) -> Self {
        let mut instance = Self {
            array: Vec::with_capacity(size),
        };
        instance.array.resize_with(size, || 0);
        return instance;
    }

    pub fn add(&mut self, idx: usize) {
        assert!(1 <= idx);
        assert!(idx <= self.array.len());
        assert!(0 == self.array[idx - 1]);
        self.array[idx - 1] = 1;
    }

    pub fn get(&self, idx: usize) -> Option<usize> {
        assert!(1 <= idx);
        assert!(idx <= self.array.len());
        assert!(1 == self.array[idx - 1]);
        let mut l_idx = idx - 1;
        let mut r_idx = idx + 1;
        while l_idx >= 1 || r_idx <= self.array.len() {
            if l_idx >= 1 {
                if 1 == self.array[l_idx - 1] {
                    return Some(idx - l_idx);
                } else {
                    l_idx -= 1;
                }
            }
            if r_idx <= self.array.len() {
                if 1 == self.array[r_idx - 1] {
                    return Some(r_idx - idx);
                } else {
                    r_idx += 1;
                }
            }
        }
        return None;
    }
}

fn format(arg: Option<usize>) -> i32 {
    if let Some(value) = arg {
        value.try_into().expect("Can't convert into i32")
    } else {
        -1
    }
}

fn main() {
    let (n, m) = read_pair::<usize, u32>();
    assert!(1 <= n);
    assert!(n <= 5 * 10 ^ 5);
    assert!(1 <= m);
    assert!(m <= 5 * 10 ^ 5);

    let mut calc = Calculator::new(n);
    for _ in 1..(m + 1) {
        let (cmd, idx) = read_pair::<String, usize>();
        match cmd.as_str() {
            "add" => calc.add(idx),
            "get" => println!("{}", format(calc.get(idx))),
            _ => panic!("Unexpected command"),
        }
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

    #[test]
    fn extended_left_to_right() {
        let mut calc = super::Calculator::new(4);
        calc.add(1);
        assert_eq!(None, calc.get(1));

        calc.add(4);
        assert_eq!(Some(3), calc.get(1));
        calc.add(3);
        assert_eq!(Some(2), calc.get(1));
        calc.add(2);
        assert_eq!(Some(1), calc.get(1));
    }

    #[test]
    fn extended_right_to_left() {
        let mut calc = super::Calculator::new(4);
        calc.add(4);
        assert_eq!(None, calc.get(4));

        calc.add(1);
        assert_eq!(Some(3), calc.get(4));
        calc.add(2);
        assert_eq!(Some(2), calc.get(4));
        calc.add(3);
        assert_eq!(Some(1), calc.get(4));
    }
}
