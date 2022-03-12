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
