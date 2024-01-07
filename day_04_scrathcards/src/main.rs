use std::fs;

#[derive(Debug)]
struct Input {
    winners: Vec<u8>,
    haves: Vec<u8>
}

impl Input {
    fn score(&self) -> usize {
        let mut count = 0;
        for w in self.winners.iter() {
            if self.haves.iter().find(|&x| x == w) == None {
                continue;
            }
            count += 1;
        }

        count
    }
}

fn parse(contents: &String) -> Vec<Input> {
    let mut result: Vec<Input> = Vec::new();

    for line in contents.lines() {
        let mut numbers = line.split(":").last().unwrap().split("|");
        let (winners,_): (Vec<_>, Vec<_>) = numbers
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .into_iter()
            .map(|s| s.parse::<u8>())
            .partition(Result::is_ok);
        let winners = winners
            .into_iter()
            .map(Result::unwrap)
            .collect();
        let (haves,_): (Vec<_>, Vec<_>) = numbers
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .into_iter()
            .map(|s| s.parse::<u8>())
            .partition(Result::is_ok);
        let haves = haves
            .into_iter()
            .map(Result::unwrap)
            .collect();

        result.push(Input{winners, haves});
    }

    result
}

fn do_part1(input: &Vec<Input>) -> usize {
    let mut result = 0;
    for card in input {
        let score = card.score() as u32;
        if score > 0 {
            result += 2_usize.pow(score - 1);
        }
    }

    result
}

fn do_part2(input: &Vec<Input>) -> usize {
    let mut result = 0;
    let mut copies = [1; 10];
    for (n, card) in input.iter().enumerate() {
        let copy = copies[0];

        for i in 0..9 {
            copies[i] = copies[i + 1];
        }
        copies[copies.len()-1] = 1;

        let winners = card.score();
        for i in 0..winners {
            copies[i] += copy;
        }
        result += copy;

        println!("card: {:?}", n+1);
        println!("copy: {:?}", copy);
        println!("copies: {:?}", copies);
        println!("winners: {:?}", winners);
        println!("result: {:?}\n", result);
    }

    result
}

fn main() {
    // let file_path = "sample1.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    assert!(part1 == 22193);

    let part2 = do_part2(&input);
    println!("Result part 2: {part2}");
    assert!(part2 == 5625994);
}
