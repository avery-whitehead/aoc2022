use std::collections::HashMap;

fn main() {
    let outcomes = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6)
    ]);
    let input = include_str!("./input.txt");
    let score = input.split("\n").fold(0, |acc, x| {
        let game: Vec<&str> = x.split_whitespace().collect();
        if game.len() == 2 {
            let opponent = game[0];
            let you = game[1];
            let outcome = outcomes.get(you).unwrap();
            // Rock
            if opponent == "A" {
                // Scissors
                if outcome == &0 {
                    acc + outcome + 3
                // Rock
                } else if outcome == &3 {
                    acc + outcome + 1
                // Paper
                } else {
                    acc + outcome + 2
                }
            // Paper
            } else if opponent == "B" {
                // Rock
                if outcome == &0 {
                    acc + outcome + 1
                // Paper
                } else if outcome == &3 {
                    acc + outcome + 2
                // Scissors
                } else {
                    acc + outcome + 3
                }
            // Scissors
            } else {
                // Paper
                if outcome == &0 {
                    acc + outcome + 2
                // Scissors
                } else if outcome == &3 {
                    acc + outcome + 3
                // Rock
                } else {
                    acc + outcome + 1
                }
            }
        } else {
            acc
        }
    });
    println!("{}", score);
}