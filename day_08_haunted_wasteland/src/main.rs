use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    directions: String,
    map: HashMap<String, (String, String)>
}

fn parse(contents: &String) -> Input {
    let sections: Vec<_> = contents.split("\n\n").collect();
    let directions = sections[0];

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in sections[1].lines() {
        let tmp: Vec<_> = line.split(" = ").collect();

        let key = tmp[0];

        let values: Vec<_> = tmp[1].split(", ").collect();
        let left = &values[0][1..];
        let right = &values[1][..3];

        map.insert(key.to_string(), (left.to_string(), right.to_string()));
    }

    Input{directions: directions.to_string(), map}
}

fn do_part1(input: &Input) -> usize {
    let Input{directions, map} = input;

    let mut key = "AAA";
    let mut steps = 0;

    loop {
        let (left, right) = map.get(key).unwrap();
        key = match directions.chars().nth(steps % directions.len()).unwrap() {
            'L' => left,
            'R' => right,
            _ => key,
        };
        steps += 1;

        if key == "ZZZ" {
            break;
        }
    }

    steps
}

// https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

// https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn do_part2(input: &Input) -> usize {
    let Input{directions, map} = input;

    let mut keys: Vec<_> = map.keys().filter(|x| x.chars().nth(2) == Some('A')).collect();
    let mut steps = 0;
    let mut result = directions.len();

    loop {
        for key in keys.clone().iter() {
            if key.chars().nth(2) == Some('Z') {
                result = lcm(result, steps);
                keys = keys.iter().filter(|&x| x != key).map(|&x| x).collect();
            }
        }

        for (i, key) in keys.clone().iter().enumerate() {
            let (left, right) = map.get(&key as &str).unwrap();
            keys[i] = match directions.chars().nth(steps % directions.len()).unwrap() {
                'L' => left,
                'R' => right,
                _ => key,
            };
        }
        steps += 1;

        if keys.len() == 0 {
            break;
        }
    }

    result
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    assert!(part1 == 16531);

    let part2 = do_part2(&input);
    println!("Result part 2: {part2}");
    assert!(part2 == 24035773251517);
}
