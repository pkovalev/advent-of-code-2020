use crate::pwd::{Rules, from_string};
use std::io::Read;
use std::time::Instant;
pub mod pwd;

fn main() {
    let mut start = Instant::now();
    let mut res = part1("resources/data.txt").unwrap();
    let mut elapsed = start.elapsed().as_millis();
    println!("Part 1: {res}. Elapsed {elapsed}  ms");
    start = Instant::now();
    res = part2("resources/data.txt").unwrap();
    elapsed = start.elapsed().as_millis();
    println!("Part 2: {res}. Elapsed {elapsed} ms");
}

fn part1(filename: &str) -> Result<u32, ()> {
    let mut f = std::fs::File::open(filename).unwrap();
    let mut buf: String = String::new();
    f.read_to_string(&mut buf).unwrap();
    let data: Vec<_> = buf.lines().map(|l| from_string(l)).collect();
    Ok(data.iter().filter(|x| check_password1(&x.0, &x.1)).count() as u32)
}

fn part2(filename: &str) -> Result<u32, ()> {
    let mut f = std::fs::File::open(filename).unwrap();
    let mut buf: String = String::new();
    f.read_to_string(&mut buf).unwrap();
    let data: Vec<_> = buf.lines().map(|l| from_string(l)).collect();
    Ok(data.iter().filter(|x| check_password2(&x.0, &x.1)).count() as u32)
}

fn check_password1(rules: &Rules, data: &str) -> bool {
    let cnt = data
        .chars()
        .filter(|c| c.eq_ignore_ascii_case(&rules.letter))
        .count();
    cnt >= rules.min as usize && cnt <= rules.max as usize
}

fn check_password2(rules: &Rules, data: &str) -> bool {
    let first = data
        .chars()
        .nth((rules.min - 1) as usize)
        .unwrap()
        .eq_ignore_ascii_case(&rules.letter);
    let second = data
        .chars()
        .nth((rules.max - 1) as usize)
        .unwrap()
        .eq_ignore_ascii_case(&rules.letter);
    first ^ second
}

#[test]
fn test_part1() {
    assert_eq!(part1("resources/test.txt"), Ok(2))
}

#[test]
fn test_part2() {
    assert_eq!(part2("resources/test.txt"), Ok(1))
}
