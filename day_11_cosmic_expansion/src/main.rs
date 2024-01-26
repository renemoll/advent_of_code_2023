use std::fs;
use std::cmp;

#[derive(Debug)]
struct Matrix<T> {
    rows: isize,
    columns: isize,
    data: Vec<T>
}

impl Matrix<u8> {
    fn parse(input: &String) -> Self {
        let data: Vec<_> = input.lines().map(str::as_bytes).collect();
        let rows = data.len() as isize;
        let columns = data[0].len() as isize;
        Matrix{
            rows,
            columns,
            data: data.concat()
        }
    }
}

impl<T> Matrix<T> where T: PartialEq<T> {
    fn find_all(&self, needle: T) -> Vec<Coordinate> {
        let mut result: Vec<Coordinate> = Vec::new();
        let mut offset = 0;
        while offset < self.data.len() {
            let pos = self.data
                .iter()
                .skip(offset)
                .position(|x| x == &needle);
            match pos {
                Some(n) => {
                    let index = offset + n;
                    offset = index + 1;
                    result.push(Coordinate::new(
                        (index  as isize) / self.columns,
                        (index  as isize) % self.columns
                    ));        
                },
                _ => break,
            }
        }

        result
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

#[derive(Debug)]
struct Input {
    galaxies: Vec<Coordinate>,
    offset_x: Vec<usize>,
    offset_y: Vec<usize>
}

fn parse(contents: &String) -> Input {
    let data = Matrix::parse(contents);
    let galaxies = data.find_all(b'#');

    let mut offset_x = vec![1;data.rows as usize];
    let mut offset_y = vec![1;data.columns as usize];
    for c in galaxies.iter() {
        offset_x[c.x as usize] = 0;
        offset_y[c.y as usize] = 0;
    }

    let mut offset = 0;
    for (n,&x) in offset_x.clone().iter().enumerate() {
        if x > 0 {
            offset += x;
        }
        offset_x[n] = offset;
    }
    offset = 0;
    for (n,&x) in offset_y.clone().iter().enumerate() {
        if x > 0 {
            offset += x;
        }
        offset_y[n] = offset;
    }

    Input{ galaxies, offset_x, offset_y}
}

fn do_part1(input: &Input) -> usize {
    let Input{galaxies, offset_x, offset_y} = input;

    let mut length = 0;
    for (n, start) in galaxies.iter().enumerate() {
        let sx = start.x as usize + offset_x[start.x as usize];
        let sy = start.y as usize + offset_y[start.y as usize];
        for other in galaxies.iter().skip(n) {
            let ox = other.x as usize + offset_x[other.x as usize];
            let oy = other.y as usize + offset_y[other.y as usize];

            let dx = cmp::max(sx, ox) - cmp::min(sx, ox);
            let dy = cmp::max(sy, oy) - cmp::min(sy, oy);
            length += dx + dy;
        }
    }

    length
}

fn main() {
    let file_path = "input.txt";
    // let file_path = "sample1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    // assert!(part1 == 9509330);

    // let part2 = do_part2(&input);
    // println!("Result part 2: {part2}");
    // assert!(part2 == 303);
}
