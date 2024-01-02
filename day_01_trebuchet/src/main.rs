use std::fs;

fn do_part1(contents: &String) -> usize {
    let mut sum = 0;
    for mut line in contents.trim().lines() {
        while !line.chars().next().unwrap().is_digit(10) {
            line = &line[1..];
        }

        while !line.chars().last().unwrap().is_digit(10) {
            line = &line[..line.len() - 1];
        }

        let first = usize::try_from(line.chars().nth(0).unwrap().to_digit(10).unwrap()).unwrap();
        let last = usize::try_from(line.chars().last().unwrap().to_digit(10).unwrap()).unwrap();
        sum += first * 10 + last;
    }
    sum
}

fn do_part2(contents: &String) -> usize {
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum = 0;
    for mut line in contents.trim().lines() {
        while !nums.iter().any(|x| line.starts_with(x)) && !line.chars().next().unwrap().is_digit(10) {
            line = &line[1..];
        }

        while !nums.iter().any(|x| line.ends_with(x)) && !line.chars().last().unwrap().is_digit(10) {
            line = &line[..line.len() - 1];
        }

        let mut first = 0;
        let mut last = 0;
        for (i, text) in nums.iter().enumerate() {
            if line.starts_with(text) {
                first = i + 1;
            }
            if line.ends_with(text) {
                last = i + 1;
            }
        }

        if line.chars().next().unwrap().is_digit(10) {
            first = usize::try_from(line.chars().nth(0).unwrap().to_digit(10).unwrap()).unwrap();
        }
        if line.chars().last().unwrap().is_digit(10) {
            last = usize::try_from(line.chars().last().unwrap().to_digit(10).unwrap()).unwrap();
        }

        sum += first * 10 + last;
    }
    sum
}
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = do_part1(&contents);
    println!("Result part 1: {part1}");
    assert!(part1 == 54940);

    let part2 = do_part2(&contents);
    println!("Result part 2: {part2}");
    assert!(part2 == 54208);
}
