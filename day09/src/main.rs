mod circle;

use crate::circle::Circle;
use hashbrown::HashMap;
use structopt::StructOpt;
use typed_arena::Arena;

#[derive(Copy, Clone, Debug, StructOpt)]
struct Opt {
    pub players: u32,
    pub marbles: u32,
}

fn main() {
    let Opt { players, marbles } = Opt::from_args();
    println!("High score: {}", get_high_score(players, marbles));
}

fn get_high_score(players: u32, marbles: u32) -> u32 {
    let mut arena = Arena::new();
    let mut circle = Circle::new(&mut arena);
    let mut scores_by_player = HashMap::new();

    let moves = (0..=marbles).zip((1..=players).cycle());

    for (marble, player) in moves {

        // Just depressing...
        if (marble + 1) % 10000 == 0 {
            println!("{}", marble + 1);
        }

        match marble % 23 {
            0 if marble != 0 => {
                *scores_by_player.entry(player).or_insert(0) += marble + circle.pop().unwrap_or(0);
            }

            _ => circle.push(marble),
        }
    }

    scores_by_player.values().cloned().max().unwrap_or_default()
}
