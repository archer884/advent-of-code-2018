use rayon::prelude::*;
use std::fmt::{self, Display};

#[derive(Copy, Clone, Debug)]
struct PolymerUnit(u8);

impl PolymerUnit {
    fn is_reactive(&self, PolymerUnit(other): PolymerUnit) -> bool {
        self.0 != other && self.0.to_ascii_lowercase() == other.to_ascii_lowercase()
    }
}

#[derive(Clone, Debug)]
struct Exclude(u8, u8);

impl Exclude {
    fn is_excluded(&self, PolymerUnit(polymer): PolymerUnit) -> bool {
        self.0 == polymer || self.1 == polymer
    }
}

impl Display for Exclude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.0 as char, self.1 as char)
    }
}

#[derive(Clone, Debug)]
struct ExclusiveReactor {
    polymer: Vec<PolymerUnit>,
    exclude: Exclude,
}

impl ExclusiveReactor {
    fn new(exclude: Exclude) -> ExclusiveReactor {
        ExclusiveReactor {
            polymer: Vec::new(),
            exclude,
        }
    }

    fn push(&mut self, polymer: PolymerUnit) {
        if self.exclude.is_excluded(polymer) {
            return;
        }

        if self.polymer.last().map(|x| x.is_reactive(polymer)).unwrap_or(false) {
            self.polymer.pop();
        } else {
            self.polymer.push(polymer);
        }
    }

    fn len(&self) -> usize {
        self.polymer.len()
    }
}

fn main() {
    let unreacted_polymer = grabinput::from_stdin().all();
    let unreacted_polymer = unreacted_polymer.trim();

    let mut reactors: Vec<_> = (b'A'..b'Z')
        .zip(b'a'..b'z')
        .map(|(x, y)| ExclusiveReactor::new(Exclude(x, y)))
        .collect();

    reactors.par_iter_mut().for_each(|reactor| {
        for reactant in unreacted_polymer.bytes().map(PolymerUnit) {
            reactor.push(reactant);
        }
    });

    reactors.sort_by_key(|x| x.len());

    if let Some(reactor) = reactors.first() {
        println!("{}: {}", reactor.exclude, reactor.len());
    } else {
        println!("Well, screw you, then.");
    }
}
