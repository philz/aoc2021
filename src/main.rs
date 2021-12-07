use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

fn oneA() {
    let file = File::open("input.txt").unwrap();
    let mut prev_depth = -1;
    let mut count_increasing = 0;
    for line_result in io::BufReader::new(file).lines() {
        let line = line_result.unwrap();
        let depth = line.parse::<i32>().unwrap();
        if prev_depth != -1 && depth > prev_depth {
            count_increasing += 1;
        }
        prev_depth = depth;
    }
    println!("{}", count_increasing);
    // let input = fs::read_to_string("input")
    //     .expect("Something went wrong reading the file");
    //
    //
    // println!("Hello, world!");
}

fn oneB() {
    let file = File::open("input.txt").unwrap();
    let mut depths: VecDeque<i32> = VecDeque::new();
    let mut count_increasing = 0;
    for line_result in io::BufReader::new(file).lines() {
        let line = line_result.unwrap();
        let depth = line.parse::<i32>().unwrap();
        depths.push_back(depth);
        if depths.len() == 4 {
            let first = depths.pop_front().unwrap();
            if depth > first {
                count_increasing += 1;
            }
        }
    }
    println!("{}", count_increasing);
}

struct Direction {
    dir: String,
    cnt: i32,
}

fn extract(input: String) -> Direction {
    let split = input.split(" ").collect::<Vec<_>>();
    let dir = String::from(split[0]);
    let cnt = split[1].parse::<i32>().unwrap();
    Direction { dir, cnt }
}

fn twoA() {
    let file = File::open("input2.txt").unwrap();
    let mut position = 0;
    let mut depth = 0;
    for line_result in io::BufReader::new(file).lines() {
        let d = extract(line_result.unwrap());
        match d.dir.as_str() {
            "up" => depth -= d.cnt, // can you go negative?
            "down" => depth += d.cnt,
            "forward" => position += d.cnt,
            _ => panic!(),
        }
    }
    println!("{}", position * depth);
}

fn twoB() {
    let file = File::open("input2.txt").unwrap();
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line_result in io::BufReader::new(file).lines() {
        let d = extract(line_result.unwrap());
        match d.dir.as_str() {
            "up" => aim -= d.cnt,
            "down" => aim += d.cnt,
            "forward" => {
                position += d.cnt;
                depth += d.cnt * aim;
            }
            _ => panic!(),
        }
    }
    println!("{}", position * depth);
}

fn threeA() {
    let file = File::open("input3.txt").unwrap();
    let mut bit_counts = Vec::new();
    for i in 0..12 {
        bit_counts.push(0);
    }
    let mut cnt = 0;
    for line_result in io::BufReader::new(file).lines() {
        cnt += 1;
        let d = line_result.unwrap();
        let x = u64::from_str_radix(&d, 2).unwrap();
        let mut i = 0;
        for i in 0..12 {
            if (x & (1 << i)) > 0 {
                bit_counts[i] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..12 {
        if bit_counts[i] as f64 >= 0.5 * (cnt as f64) {
            gamma += (1 << i);
        } else {
            epsilon += (1 << i);
        }
    }
    println!("{:#?} {:#?} {:#?} {}", bit_counts, cnt, gamma, epsilon);
    println!("{}", gamma * epsilon);
}

fn commonBit(nums: &[i32], bit: i32) -> bool {
    let total_count = nums.iter().count();
    let bit_count = nums.iter().filter(|x| {
        bitIsSet(**x, bit)
    }).count();
    return bit_count * 2 >= total_count;
}

fn bitIsSet(x: i32, bit: i32) -> bool {
    (x & (1 << bit)) > 0
}

fn threeB() {
    let file = File::open("input3.txt").unwrap();
    let values: Vec<i32> = io::BufReader::new(file).lines().map(|l| {
        i32::from_str_radix(&l.unwrap(), 2).unwrap()
    }).collect();
    let oxygen;
    {
        let mut filtered = values.clone();
        let mut i = 11;
        while filtered.len() > 1 {
            let c = commonBit(&filtered, i);
            filtered = filtered.into_iter().filter(|x| { c == bitIsSet(*x, i) }).collect();
            i = i - 1;
        }
        oxygen = filtered[0];
    }

    let co2;
    {
        let mut filtered = values.clone();
        let mut i = 11;
        while filtered.len() > 1 {
            let c = commonBit(&filtered, i);
            filtered = filtered.into_iter().filter(|x| { c != bitIsSet(*x, i) }).collect();
            i = i - 1;
        }
        co2 = filtered[0];
    }

    // let co2;
    // {
    //     let mut filtered = values.into_iter().collect::<Vec<i32>>();
    //     let i = 11;
    //     while filtered.len() > 1 {
    //         let c = commonBit(&filtered, i);
    //         filtered = filtered.into_iter().filter(|x| { c != bitIsSet(*x, i) }).collect();
    //     }
    //     co2 = filtered[0];
    // }
    println!("{} {} {}", oxygen, co2, oxygen * co2);
    //     cnt += 1;
    //     let d = line_result.unwrap();
    //     let x = u64::from_str_radix(&d, 2).unwrap();
    //     let mut i = 0;
    //     for i in 0..12 {
    //         if (x & (1 << i)) > 0 {
    //             bit_counts[i] += 1;
    //         }
    //     }
    // }
    // let mut gamma = 0;
    // let mut epsilon = 0;
    // for i in 0..12 {
    //     if bit_counts[i] as f64 >= 0.5 * (cnt as f64) {
    //         gamma += (1 << i);
    //     } else {
    //         epsilon += (1 << i);
    //     }
    // }
    // println!("{:#?} {:#?} {:#?} {}", bit_counts, cnt, gamma, epsilon);
    // println!("{}", gamma * epsilon);
}

struct BingoNumber {
    n: i32,
    is_set: bool,
}

struct BingoBoard {
    numbers: [BingoNumber; 25],
}

impl BingoBoard {
    pub unsafe fn new(input: &Vec<i32>) -> Self {
        let mut numbers: [BingoNumber; 25] = std::mem::uninitialized(); // lol!
        assert_eq!(input.len(), 25);
        for i in 0..input.len() {
            numbers[i] = BingoNumber { n: input[i], is_set: false };
        }
        Self { numbers }
    }

    pub fn done(&self) -> bool {
        // Rows
        for i in 0..5 {
            let mut bingo = true;
            for j in 0..5 {
                if !self.numbers[i * 5 + j].is_set {
                    bingo = false;
                    break;
                }
            }
            if bingo {
                return true;
            }
        }

        // Columns
        for i in 0..5 {
            let mut bingo = true;
            for j in 0..5 {
                if !self.numbers[i + j * 5].is_set {
                    bingo = false;
                    break;
                }
            }
            if bingo {
                return true;
            }
        }
        return false;
    }

    pub fn mark(&mut self, x: i32) -> Option<i32> {
        // can only win once
        if self.done() { return None };
        for i in 0..self.numbers.len() {
            if self.numbers[i].n == x {
                self.numbers[i].is_set = true;
                if self.done() {
                    return Some(self.score(x));
                }
            }
        }
        return None;
    }

    pub fn score(&self, x: i32) -> i32 {
        self.numbers.iter().filter(|x| { !x.is_set }).map(|x| { x.n }).sum::<i32>() * x
    }
}

fn four() {
    let file = File::open("input4.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|l| { l.unwrap()}).collect();
    let called_numbers = lines[0].split(',').map(|x| { x.parse::<i32>().unwrap() });

    let mut boards: Vec<BingoBoard> = Vec::new();
    let numbers: Vec<i32> = lines[1..lines.len()].join(" ").as_str().split_whitespace().map(|x| { x.parse::<i32>().unwrap()}).collect();
    let mut stage: Vec<i32> = Vec::new();
    for x in numbers.iter() {
        stage.push(*x);
        if stage.len() == 25 {
            unsafe { boards.push(BingoBoard::new(&stage)); }
            stage.clear();
        }
    }

    let mut has_seen_first = false;
    let mut remaining_boards = boards.len();
    for x in called_numbers {
        for board in boards.iter_mut() {
            let r = board.mark(x);
            if r.is_some() {
                remaining_boards -= 1;
                if !has_seen_first {
                    println!("first win board {}", r.unwrap());
                    has_seen_first = true;
                }
                if remaining_boards == 0 {
                    println!("last win board {}", r.unwrap());
                }
            }
        }
    }
}

fn five() {
    let file = File::open("input5.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|l| { l.unwrap()}).collect();
    lines[0]

    let called_numbers = lines[0].split(',').map(|x| { x.parse::<i32>().unwrap() });

    let mut boards: Vec<BingoBoard> = Vec::new();
    let numbers: Vec<i32> = lines[1..lines.len()].join(" ").as_str().split_whitespace().map(|x| { x.parse::<i32>().unwrap()}).collect();

}

fn main() {
    if (false) {
        oneA();
        oneB();
        twoA();
        twoB();
        threeA();
        threeB();
        four();
    }
    five();
}
