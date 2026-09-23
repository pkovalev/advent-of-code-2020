use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut res = part1("resources/data.txt").unwrap();
    println!("Part 1: {res}");
    res = part2("resources/data.txt").unwrap();
    println!("Part 2: {res}");
}

fn part1(filename: &str) -> Result<u32, ()> {
    let mut f = File::open(filename).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let numbers: Vec<u32> = buf.lines().map(|line| u32::from_str(line).unwrap()).collect();
    let expected: u32 = 2020;
    for i in 0..numbers.len() - 1 {
        for j in i..numbers.len() {
            if numbers[i] + numbers [j] == expected {
                return Ok(numbers[i]*numbers[j]);
            }
        }
    }
    Err(())
}

fn part2(filename: &str) -> Result<u32, ()> {
    let mut f = File::open(filename).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let numbers: Vec<u32> = buf.lines().map(|line| u32::from_str(line).unwrap()).collect();
    let expected: u32 = 2020;
    for i in 0..numbers.len() - 2 {
        for j in i..numbers.len() - 1 {
            for k in j..numbers.len() {
                if numbers[i] + numbers [j] + numbers[k] == expected {
                    return Ok(numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
    Err(())
}

#[test]
fn test_part1() {
    assert_eq!(part1("resources/test.txt"), Ok(514579))
}

#[test]
fn test_part2() {
    assert_eq!(part2("resources/test.txt"), Ok(241861950))
}