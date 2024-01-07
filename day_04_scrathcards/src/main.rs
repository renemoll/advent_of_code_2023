use std::fs;

#[derive(Debug)]
struct Input {
    winners: Vec<u8>,
    haves: Vec<u8>
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
    for i in input {
        let Input{winners, haves} = i;
        let mut score = 0;

        for w in winners {
            if haves.iter().find(|&x| x == w) == None {
                continue;
            }

            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
        result += score;
    }

    result
}


fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
//22193
}
