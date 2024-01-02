use std::fs;

fn parser(contents: &String) -> usize {
    let mut sum = 0;
    for line in contents.trim().lines() {
        let mut num = String::new();
        for ch in line.chars() {
            if ch.is_digit(10) {
                num.push(ch);
            }
        }

        // note: assuming ASCII characters here.
        let mut res = String::new();
        res.push(num.chars().nth(0).unwrap());
        res.push(num.chars().last().unwrap());
        let tmp: usize = res.parse().expect("a number");
        sum += tmp;
    }
    sum
}

fn part2(contents: &String) -> usize {
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

        let res = first * 10 + last;
        sum += res;
    }
    sum
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = parser(&contents);
    println!("Result part 1: {part1}");
    // assert -> 54940

    let part2 = part2(&contents);
    println!("Result part 2: {part2}");
    // assert -> 54208
}
