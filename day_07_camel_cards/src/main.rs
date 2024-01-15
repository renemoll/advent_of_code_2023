use std::fs;

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: usize
}

#[derive(Debug)]
struct Input {
    hands: Vec<Hand>
}

fn parse(contents: &String) -> Input {
    let mut hands = Vec::new();
    for l in contents.lines() {
        let sections: Vec<_> = l.split(" ").collect();
       hands.push(Hand{hand: sections[0].to_string(), bid: sections[1].parse().unwrap()});
    }

    Input{hands}
}

fn calculate_score(input: &Input, joker: u8) -> usize {
    let mut result: Vec<_> = Vec::new();
    for hand in input.hands.iter() {
        // println!("hand: {0}, bid: {1}", hand.hand, hand.bid);

        let draw = hand.hand.bytes().map(|x|
            match x {
                b'A' => 14, 
                b'K' => 13,
                b'Q' => 12,
                b'J' => joker,
                b'T' => 10,
                _ => x - 48,
            }
        ).collect::<Vec<_>>();

        // Frequency of each kind of card (14 kind of cards.)
        let mut frequency = [0; 15];
        draw.iter().for_each(|&x| frequency[x as usize] += 1);

        let j = frequency[1];
        frequency[1] = 0;
        frequency.sort();
        frequency.reverse();
        frequency[0] += j;

        /*
            frequency:
                best case:  four of a kind; 2 different types.
                worst case: high card; 5 different types.
                5 cards, 4 bits per card. 
                 -> 20 bits.

            card values:
                5 cards, 4 bits per card.
                 -> 20 bits.

            So we need a datatype of at least 40 bits.
        */

        let mut score: u64 = 0;

        for f in frequency.iter().take(5) {
            score = (score << 4) | f;
        }

        for d in draw {
            score = (score << 4) | (d as u64);
        }

        result.push((score, hand.bid));
    }

    result.sort();
    let mut score = 0; 
    for (n, (_, bid)) in result.iter().enumerate() {
        score += (n + 1) * bid;
    }

    score
}

fn do_part1(input: &Input) -> usize {
    calculate_score(input, 11)
}

fn do_part2(input: &Input) -> usize {
    calculate_score(input, 1)
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse(&contents);

    let part1 = do_part1(&input);
    println!("Result part 1: {part1}");
    assert!(part1 == 256448566);

    let part2 = do_part2(&input);
    println!("Result part 2: {part2}");
    assert!(part2 == 254412181);
}
