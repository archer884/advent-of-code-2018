#[derive(Copy, Clone, Debug)]
struct PolymerUnit(u8);

impl PolymerUnit {
    fn is_reactive(&self, PolymerUnit(other): PolymerUnit) -> bool {
        self.0 != other && self.0.to_ascii_lowercase() == other.to_ascii_lowercase()
    }
}

#[derive(Clone, Debug, Default)]
struct Reactor(Vec<PolymerUnit>);

impl Reactor {
    fn new() -> Reactor {
        Default::default()
    }

    fn push(&mut self, polymer: PolymerUnit) {
        if self.0.last().map(|x| x.is_reactive(polymer)).unwrap_or(false) {
            self.0.pop();
        } else {
            self.0.push(polymer);
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn main() {
    let unreacted_polymer = grabinput::from_stdin().all();
    let unreacted_polymer = unreacted_polymer.trim();

    let mut reactor = Reactor::new();
    for reactant in unreacted_polymer.bytes().map(PolymerUnit) {
        reactor.push(reactant);
    }

    println!("{}", reactor.len());
}
