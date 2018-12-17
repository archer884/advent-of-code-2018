mod circle;

use crate::circle::Circle;
use hashbrown::HashMap;
use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
struct Opt {
    pub players: u32,
    pub last_marble: u32,
}

fn main() {
    let opt = Opt::from_args();

    let mut circle = Circle::default();
    let mut scores_by_player = HashMap::new();

    let moves = (0..=opt.last_marble).zip((1..=opt.players).cycle());

    for (marble, player) in moves {
        match marble % 23 {
            0 if marble != 0 => {
                *scores_by_player.entry(player).or_insert(0) += marble + circle.pop().unwrap_or(0);
            }

            _ => circle.push(marble),
        }
    }

    if let Some(score) = scores_by_player.values().max() {
        println!("High score: {}", score);
    }
}
