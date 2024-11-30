use std::fs;
use std::collections::HashMap;

struct Game {
    score: i32,
}

impl Game {
    fn round(&mut self, line: &String) -> () {
        let round: Vec<&str> = line.split(" ").collect();
        let opponent = round[0];
        let player = round[1];
        self.score += self.calculate_win(player, opponent);
    }

    fn second_round(&mut self, line: &String) -> () {
        let round: Vec<&str> = line.split(" ").collect();
        let opponent = round[0];
        let player = round[1];
        self.score += self.calculate_real_win(player, opponent);
    }

    fn calculate_real_win(&self, player: &str, opponent: &str) -> i32 {
        let a = HashMap::from([
            ("win", "Y"),
            ("lose", "Z"),
            ("draw", "X"),
        ]);

        let b = HashMap::from([
            ("win", "Z"),
            ("lose", "X"),
            ("draw", "Y"),
        ]);

        let c = HashMap::from([
            ("win", "X"),
            ("lose", "Y"),
            ("draw", "Z"),
        ]);


        if player == "X" {
            // Need to lose
            if opponent == "A" {
                return 0 + self.score_for_hand(a[&"lose"]);
            } else if opponent == "B" {
                return 0 + self.score_for_hand(b[&"lose"]);
            } else if opponent == "C" {
                return 0 + self.score_for_hand(c[&"lose"]);
            } else {
                return 0;
            }
        } else if player == "Y" {
            // Need to draw
            if opponent == "A" {
                return 3 + self.score_for_hand(a[&"draw"]);
            } else if opponent == "B" {
                return 3 + self.score_for_hand(b[&"draw"]);
            } else if opponent == "C" {
                return 3 + self.score_for_hand(c[&"draw"]);
            } else {
                return 3;
            }

        } else if player == "Z" {
            // Need to win
            if opponent == "A" {
                return 6 + self.score_for_hand(a[&"win"]);
            } else if opponent == "B" {
                return 6 + self.score_for_hand(b[&"win"]);
            } else if opponent == "C" {
                return 6 + self.score_for_hand(c[&"win"]);
            } else {
                return 0;
            }

        } else {
            return 0;
        }
    }

    fn calculate_win(&self, player: &str, opponent: &str) -> i32 {

        // Rock vs Rock
        if opponent == "A" && player == "X" {
            return 3 + self.score_for_hand(player)
        }
        // Rock vs Paper
        else if opponent == "A" && player == "Y" {
            return 6 + self.score_for_hand(player)
        }
        // Rock vs Scissors
        else if opponent == "A" && player == "Z" {
            return 0 + self.score_for_hand(player)
        }
        // Paper vs Rock
        else if opponent == "B" && player == "X" {
            return 0 + self.score_for_hand(player)
        }
        // Paper vs Paper
        else if opponent == "B" && player == "Y" {
            return 3 + self.score_for_hand(player)
        }
        // Paper vs Scissors
        else if opponent == "B" && player == "Z" {
            return 6 + self.score_for_hand(player)
        }
        // Scissors vs Rock
        else if opponent == "C" && player == "X" {
            return 6 + self.score_for_hand(player)
        }
        // Scissors vs Paper
        else if opponent == "C" && player == "Y" {
            return 0 + self.score_for_hand(player)
        }
        // Scissors vs Scissors
        else if opponent == "C" && player == "Z" {
            return 3 + self.score_for_hand(player)
        } else {
            return 0
        }
    }

    fn score_for_hand(&self, hand: &str) -> i32 {

        match hand {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        }
    }
}


fn main() {

    let contents = fs::read_to_string("puzzle.txt")
        .expect("Something went wrong reading the file");

    let mut game = Game {
        score: 0,
    };

    let mut second_game = Game {
        score: 0,
    };

    for line in contents.lines() {
        game.round(&line.to_string());
        second_game.second_round(&line.to_string());
    }

    println!("Total score : {}", game.score);
    println!("Total score : {}", second_game.score);

}
