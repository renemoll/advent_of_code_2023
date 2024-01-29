use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct Input<'a> {
    sections: Vec<&'a str>,
    lengths: Vec<usize>,
}

fn parse(input: &str) -> Vec<Input> {
    let mut result: Vec<Input> = Vec::new();

    for raw in input.lines() {
        let l = raw.split(' ').collect::<Vec<_>>();
        let mut sections = l[0].split('.')
            .filter(|&x| x.len() > 0)
            .collect::<Vec<_>>();
        let mut numbers: Vec<usize> = l[1].split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>();

        let mut offset = 0;
        for (n, &s) in sections.clone().iter().enumerate() {
            let length = s.chars().filter(|&x| x == '#').count();
            if length == s.len() {
                let index_num = numbers.iter().position(|&x| x == length).unwrap();
                numbers.remove(index_num);
                sections.remove(n - offset);
                offset += 1;
            }
        }
        result.push(Input{sections: sections, lengths: numbers});
    }

    result
}

fn do_part1(input: &Vec<Input>) -> usize {
    println!("{:?}", input);

    let mut result = 0;
    for i in 0..input.len() {
        if input[i].sections.len() == input[i].lengths.len() {
            // number of sections match the numbers of numbers
            for (s, n) in zip(input[i].sections.iter(), input[i].lengths.iter()) {
                println!("- section: {:?}, length: {:?}, result: {:?}", s, n, s.len() / n);
                result += s.len() / n;
            }
        } else {
            println!("= section: {:?}, length: {:?}, result: {:?}", input[i].sections, input[i].lengths, 0);
        }
    }

    result
}

fn main() {
    // let file_path = "input.txt";
    let file_path = "sample1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    // assert!(part1 == 6701);

    // let part2 = do_part2(&input);
    // println!("Result part 2: {part2}");
    // assert!(part2 == 303);
}
