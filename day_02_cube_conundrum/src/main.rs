use std::fs;

#[derive(Debug)]
struct Game {
    red: usize,
    blue: usize,
    green: usize
}

impl Game {
    fn valid(&self, other: &Game) -> bool {
        return self.red <= other.red
            && self.blue <= other.blue
            && self.green <= other.green;
    }
}

fn game_from_str(game: &str) -> Game {
    let mut g = Game{red: 0, blue: 0, green: 0};
    for dice in game.split(",") {
        let mut x = dice.trim().split(" ");
        let n = x.next().unwrap().parse().unwrap();

        if dice.find("red").is_some() {
            g.red = n;
        }
        if dice.find("blue").is_some() {
            g.blue = n;
        }
        if dice.find("green").is_some() {
            g.green = n;
        }
    }
    g
}

fn do_part1(contents: &String) -> usize {
    let max = Game {
        red: 12,
        blue: 14,
        green: 13
    };

    let mut sum = 0;

    for line in contents.trim().lines() {
        let mut it = line.split(':');
        let id: usize = it.next().unwrap().split(" ").last().unwrap().parse().unwrap();

        let draws = it.last().unwrap().split(";");
        let mut valid = true;
        for draw in draws {
            valid = valid && game_from_str(draw).valid(&max);
        }

        if valid {
            sum += id;
        }
    }
    sum
}

fn do_part2(contents:&String) -> usize {
    let mut sum = 0;

    for line in contents.trim().lines() {
        let it = line.split(':');
        let draws = it.last().unwrap().split(";");

        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for draw in draws {
            let g = game_from_str(draw);
            blue = blue.max(g.blue);
            red = red.max(g.red);
            green = green.max(g.green)
        }
        sum += red * green * blue;
    }
    sum
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = do_part1(&contents);
    println!("Result part 1: {part1}");
    assert!(part1 == 2600);

    let part2 = do_part2(&contents);
    println!("Result part 2: {part2}");
    assert!(part1 == 86036);
}
