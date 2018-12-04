use hashbrown::HashMap;

struct IdMap(HashMap<u8, usize>);

impl IdMap {
    fn from_id(s: &str) -> IdMap {
        let mut map = HashMap::default();
        s.bytes().for_each(|u| *map.entry(u).or_insert(0) += 1);
        IdMap(map)
    }

    fn has_two(&self) -> bool {
        self.0.values().any(|&x| x == 2)
    }

    fn has_three(&self) -> bool {
        self.0.values().any(|&x| x == 3)
    }
}

fn main() {
    let input: Vec<_> = grabinput::from_stdin()
        .map(|s| s.trim().to_string())
        .collect();

    // part one
    let id_maps: Vec<_> = input.iter().map(|s| IdMap::from_id(s)).collect();
    let (two, three) = id_maps.iter().fold((0, 0), |(two, three), b| {
        (
            if b.has_two() { two + 1 } else { two },
            if b.has_three() { three + 1 } else { three },
        )
    });

    println!("{}", two * three);

    // part two
    if let Some((_, left, right)) = comparisons(&input, &input)
        .map(|(left, right)| (edit_distance(left, right), left, right))
        .find(|x| x.0 == 1)
    {
        println!("{}, {}", left, right);
        println!("Shared letters: {}", shared_letters(left, right));
    };
}

fn comparisons<'a, T>(left: &'a [T], right: &'a [T]) -> impl Iterator<Item = (&'a T, &'a T)> + 'a {
    left.iter()
        .flat_map(move |left| right.iter().map(move |right| (left, right)))
}

fn edit_distance(left: &str, right: &str) -> usize {
    left.bytes()
        .zip(right.bytes())
        .filter(|(left, right)| left != right)
        .count()
}

fn shared_letters(left: &str, right: &str) -> String {
    left.bytes()
        .zip(right.bytes())
        .filter(|(left, right)| left == right)
        .map(|u| u.0 as char)
        .collect()
}
