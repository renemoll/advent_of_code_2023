use std::fs;
use std::ops;

struct Input {
    seeds: Vec<usize>,
    mapping: Vec<Vec<(usize, std::ops::Range<usize>)>>
}

fn parse(contents: &String) -> Input {
    let sections: Vec<_> = contents.split("\n\n").collect();

    let seeds: Vec<_> = sections[0].split(": ").collect::<Vec<_>>()[1].split(" ").collect();
    let seeds: Vec<_> = seeds.iter().map(|x| x.parse::<usize>()).map(Result::unwrap).collect();

    let mut mapping: Vec<Vec<_>> = Vec::new();
    for i in 1..sections.len() {
        let rows: Vec<_> = sections[i].split("\n").collect();
        let map: Vec<_> = rows[1..]
            .iter()
            .map(|x|
                x.split(" ")
                .map(|x| x.parse::<usize>())
                .map(Result::unwrap)
                .collect::<Vec<_>>()
            )
            .map(|x| (
                x[0],
                ops::Range{start: x[1], end: x[1] + x[2]}
            ))
            .collect::<Vec<_>>();
            mapping.push(map)
    }

    Input{seeds, mapping}
}

fn do_part1(input: &Input) -> usize {
    let Input{seeds, mapping} = input;

    let mut locations = Vec::new();
    for seed in seeds {
        let mut location = *seed;
        for m in mapping {
            for (dest, src) in m {
                if src.contains(&location) {
                    location = dest + location - src.start;
                    break;
                }
            }
        }
        locations.push(location);
    }

    *locations.iter().min().unwrap()
}

fn do_part2(input: &Input) -> usize {
    let Input{seeds, mapping} = input;

    let seed_ranges: Vec<_> = seeds.chunks(2).collect();
    let mut locations = Vec::new();
    for i in seed_ranges {
        for j in 0..i[1] {
            let seed = i[0] + j;
            let mut location = seed;
            for m in mapping {
                for (dest, src) in m {
                    if src.contains(&location) {
                        location = dest + location - src.start;
                        break;
                    }
                }
            }
            locations.push(location);
        }
    }

    *locations.iter().min().unwrap()
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    assert!(part1 == 240320250);

    let part2 = do_part2(&input);
    println!("Result part 2: {part2}");
    assert!(part2 == 28580589);
}
