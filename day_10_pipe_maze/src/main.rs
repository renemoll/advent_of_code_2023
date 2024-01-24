use std::fs;
use std::ops::Add;
use std::ops::Sub;


#[derive(Debug)]
struct Matrix<T> {
    rows: isize,
    columns: isize,
    data: Vec<T>
}

impl Matrix<Pipe> {
    fn parse(input: &String) -> Self {
        let data: Vec<_> = input.lines().map(str::as_bytes).collect();
        let rows = data.len() as isize;
        let columns = data[0].len() as isize;
        Matrix{
            rows,
            columns,
            data: data.concat().iter().map(char2pipe).collect()
        }
    }
}

impl<T> Matrix<T> {
    // X maps on rows, Y maps on cols
    fn get(&self, c: &Coordinate) -> &T {
        &self.data[(c.x * self.columns + c.y) as usize]
    }

    // fn set(&mut self, row: usize, col: usize, data: T) {
    //     self.data[row * self.columns + col] = data;
    // }
}

impl<T> Matrix<T> where T: PartialEq<T> {
    fn find(&self, needle: T) -> Coordinate {
        let pos = self.data.iter().position(|x| x == &needle).unwrap();
        Coordinate::new(
            (pos as isize) / self.columns,
            (pos as isize) % self.columns
        )
    }
}

#[derive(Debug, PartialEq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

fn char2pipe(c: &u8) -> Pipe {
    match c {
        b'|' => Pipe::NS,
        b'-' => Pipe::EW,
        b'L' => Pipe::NE,
        b'J' => Pipe::NW,
        b'7' => Pipe::SW,
        b'F' => Pipe::SE,
        b'.' => Pipe::Ground,
        b'S' => Pipe::Start,
        _ => panic!("Unkown tile"),
    }
}

fn pipe2directions(p: &Pipe) -> Vec<Coordinate> {
    match p {
        Pipe::NS => vec![Coordinate::new(-1, 0), Coordinate::new(1, 0)],
        Pipe::EW => vec![Coordinate::new(0, 1), Coordinate::new(0, -1)],
        Pipe::NE => vec![Coordinate::new(-1, 0), Coordinate::new(0, 1)],
        Pipe::NW => vec![Coordinate::new(-1, 0), Coordinate::new(0, -1)],
        Pipe::SW => vec![Coordinate::new(1, 0), Coordinate::new(0, -1)],
        Pipe::SE => vec![Coordinate::new(1, 0), Coordinate::new(0, 1)],
        Pipe::Ground => vec![],
        Pipe::Start => vec![
            Coordinate::new(1, 0),
            Coordinate::new(-1, 0),
            Coordinate::new(0, 1),
            Coordinate::new(0, -1)],
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize 
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Coordinate{x, y}
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Coordinate::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug)]
struct Input {
    pipes: Matrix<Pipe>,
    movement: Matrix<Vec<Coordinate>>,
    start: Coordinate
}

fn parse(contents: &String) -> Input {
    let data = Matrix::parse(contents);
    let directions = Matrix {
        rows: data.rows,
        columns: data.columns,
        data: data.data.iter().map(pipe2directions).collect()
    };

    let start = data.find(Pipe::Start);
    Input{
        pipes: data,
        movement: directions,
        start: start
    }
}

fn do_part1(input: &Input) -> usize {
    let Input{pipes, movement, start} = input;

    let mut count = 0;
    let mut seen : Vec<Coordinate> = Vec::new();
    let mut next : Vec<Coordinate> = Vec::new();
    next.push(*start);

    loop {
        let current = next.pop().unwrap();
        seen.push(current);

        let options = movement
            .get(&current)
            .iter()
            .map(|&rel| current + rel)
            .filter(|&c| c.x >= 0 && c.y >= 0 && *pipes.get(&c) != Pipe::Ground)
            .collect::<Vec<_>>();

        for opt in options {
            let new = seen.iter().find(|&x| *x == opt) == None;
            let unique = next.iter().find(|&x| *x == opt) == None;
            if new && unique {
                next.push(opt);
            }
        }

        count += 1;
        if next.len() == 0 {
            break;
        }
    }

    count / 2
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    // assert!(part1 == 6701);

    // let part2 = do_part(&input);
    // println!("Result part 2: {part2}");
    // assert!(part2 == 900);
}
