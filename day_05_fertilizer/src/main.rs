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

    // println!("{:?}", seeds);
    // println!("{:?}", mapping);
    let mut locations = Vec::new();
    for seed in seeds {
        // let seed = seeds[0];
        let mut location = *seed;
        // println!("start seed: {:?}\n", location);
        for m in mapping {
            for (dest, src) in m {
                // println!("dest: {:?}", dest);
                // println!("src: {:?}", src);

                if src.contains(&location) {
                    location = dest + location - src.start;
                    // println!("seed: {:?}\n", seed);
                    break;
                }
            }
        }
        // println!("location: {:?}", location);
        locations.push(location);
    }

    *locations.iter().min().unwrap()
    // 0
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    // assert!(part1 == 240320250);

    // let part2 = do_part2(&input);
    // println!("Result part 2: {part2}");
    // assert!(part2 == 91031374);
}
