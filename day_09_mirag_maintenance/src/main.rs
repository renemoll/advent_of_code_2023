use std::fs;

#[derive(Debug)]
struct Sequence {
    data: Vec<isize>
}

impl Sequence {
    fn diff(&self) -> Sequence {
        return Sequence{
            data: self.data
                .windows(2)
                .map(|vs| {
                    let [x, y] = vs else { unreachable!() };
                    y - x
                })
                .collect()
        }
    }

    fn all_zero(&self) -> bool {
        if self.data.is_empty() {
            return true
        }

        for v in self.data.iter() {
            if *v > 0 || *v < 0 {
                return false
            }
        }

        true
    }
}

#[derive(Debug)]
struct Input {
    sequences: Vec<Sequence>
}

fn parse(contents: &String) -> Input {
    let mut result: Vec<Sequence> = Vec::new();

    for line in contents.lines() {
        result.push(Sequence{
            data: line.split(" ").map(|x| x.parse().unwrap()).collect()
        })
    }

    Input{sequences: result}
}

fn process(xs: &Sequence) -> isize {
    if xs.all_zero() {
        return 0;
    }

    let d = xs.diff();
    return xs.data.last().unwrap() + process(&d);
}

fn do_part1(input: &Input) -> isize {
    let mut result = 0;
    for s in input.sequences.iter() {
        result += process(s);
    }

    result
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    assert!(part1 == 1987402313);

    // let part2 = do_part2(&input);
    // println!("Result part 2: {part2}");
    // assert!(part2 == 24035773251517);
}
