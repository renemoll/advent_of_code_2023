use std::fs;

#[derive(Debug)]
struct Matrix<T> {
    rows: usize,
    columns: usize,
    data: Vec<T>
}

impl Matrix<u8> {
    fn parse(input: &String) -> Self {
        let data: Vec<_> = input.lines().map(str::as_bytes).collect();
        let rows = data.len();
        let columns = data[0].len();
        Matrix{rows, columns, data: data.concat()}
    }
}

impl<T> Matrix<T> {
    fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.columns + col]
    }

    fn set(&mut self, row: usize, col: usize, data: T) {
        self.data[row * self.columns + col] = data;
    }
}

struct Input {
    indices: Matrix<usize>,
    numbers: Vec<usize>,
    symbols: Vec<(isize, isize, u8)>
}

fn parse(contents: &String) -> Input {
    let data = Matrix::parse(contents);
    let mut indices = Matrix {rows: data.rows, columns: data.columns, data: vec![0; data.rows * data.columns]};
    let mut numbers = vec![0];
    let mut symbols: Vec<(isize, isize, u8)> = Vec::new();

    for r in 0..data.rows {
        let mut num = 0;
        for c in 0..data.columns {
            let el = data.get(r, c);

            if el.is_ascii_digit() {
                num = 10 * num + (el.wrapping_sub(b'0') as usize);
                indices.set(r, c, numbers.len());
            } else if num > 0 {
                numbers.push(num);
                num = 0;
            }

            if !el.is_ascii_digit() && *el != b'.' {
                symbols.push((r.try_into().unwrap(), c.try_into().unwrap(), *el));
            }
        }

        if num > 0 {
            numbers.push(num);
        }
    }

    Input{indices, numbers, symbols}
}

fn do_part1(input: &Input) -> usize {
    let Input{indices, numbers, symbols} = input;

    let mut result = 0;
    let diag = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    for (r, c, _) in symbols {
        let mut processed: Vec<usize> = Vec::new();
        for (dr, dc) in diag {
            let x = (r + dr) as usize;
            let y = (c + dc) as usize;
            let idx = *indices.get(x, y);
            if idx > 0 && !processed.contains(&idx) {
                result += numbers[idx];
                processed.push(idx);
            }
        }
    }

    result
}

fn do_part2(input: &Input) -> usize {
    let Input{indices, numbers, symbols} = input;

    let mut result = 0;
    let diag = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    for (r, c, s) in symbols {
        if *s != b'*' {
            continue;
        }

        let mut ratio = 1;
        let mut processed: Vec<usize> = Vec::new();
        for (dr, dc) in diag {
            let x = (r + dr) as usize;
            let y = (c + dc) as usize;
            let idx = *indices.get(x, y);
            if idx > 0 && !processed.contains(&idx) {
                ratio = ratio * numbers[idx];
                processed.push(idx);
            }
        }
        if processed.len() > 1 {
            result += ratio;
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
    assert!(part1 == 546563);

    let part2 = do_part2(&input);
    println!("Result part 2: {part2}");
    assert!(part2 == 91031374);
}
