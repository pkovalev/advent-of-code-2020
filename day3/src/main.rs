use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_input("resources/data.txt");
    let mut res = part1(&data);
    println!("Part 1: {res}");
    res = part2(&data);
    println!("Part 2: {res}");
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut f = File::open(filename).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf.lines().map(|line| line.chars().collect()).collect()
}

fn calc(data: &Vec<Vec<char>>, step_right: usize, step_down: usize) -> usize {
    let sz = data.first().unwrap().len();
    let mut counter: usize = 0;
    let mut i = 0;
    while i * step_down < data.len() {
        if data[i * step_down][i * step_right % sz].eq_ignore_ascii_case(&'#') {
            counter += 1;
        }
        i += 1;
    }
    counter
}

fn part1(data: &Vec<Vec<char>>) -> usize {
    calc(data, 3, 1)
}

fn part2(data: &Vec<Vec<char>>) -> usize {
    let mut counter: usize = 1;
    counter *= calc(data, 1, 1);
    counter *= calc(data, 3, 1);
    counter *= calc(data, 5, 1);
    counter *= calc(data, 7, 1);
    counter *= calc(data, 1, 2);
    counter
}

#[test]
fn test_part1() {
    assert_eq!(part1(&read_input("resources/test.txt")), 7);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&read_input("resources/test.txt")), 336);
}
