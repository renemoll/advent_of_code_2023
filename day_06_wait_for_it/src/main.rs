use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize
}

struct Input {
    races: Vec<Race>
}

fn parse(contents: &String) -> Input {
    let mut data: Vec<_> = Vec::new();
    for l in contents.lines() {
        let mut nums = l.split(":").collect::<Vec<_>>()[1].trim().split(" ").collect::<Vec<_>>();
        nums.retain(|&x| x != "");
        data.push(nums);
    }

    let races = zip(data[0].iter(), data[1].iter())
        .map(|(x,y)| Race{time: x.parse().unwrap(), distance: y.parse().unwrap()})
        .collect::<Vec<_>>();
    Input{races}
}

fn do_part1(input: &Input) -> usize {
    let mut result = 1;
    for race in input.races.iter() {
        let t = 1..race.time;
        let d = t.map(|x| (race.time - x) * x)
            .filter(|&x| x > race.distance)
            .collect::<Vec<_>>();
        result = result * d.len();
    }

    result
}

fn do_part2(input: &Input) -> usize {
    let mut t = String::new();
    let mut d = String::new();
    for race in input.races.iter() {
        t += &race.time.to_string();
        d += &race.distance.to_string();
    }

    let time: usize = t.parse().unwrap();
    let distance: usize = d.parse().unwrap();

    let t = 1..time;
    let d = t.map(|x| (time - x) * x)
        .filter(|&x| x > distance)
        .collect::<Vec<_>>();

    d.len()
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    assert!(part1 == 1624896);

    let part2 = do_part2(&input);
    println!("Result part 2: {part2}");
    assert!(part2 == 32583852);
}
