use std::collections::HashSet;

fn main() {
    let state_changes: Vec<_> = grabinput::from_args()
        .with_fallback()
        .filter_map(as_integer)
        .collect();

    let mut set = HashSet::new();
    let mut state = 0;

    for change in state_changes.iter().cycle() {
        if !set.insert(state) {
            break;
        }

        state += change;        
    }

    println!("{}", state);
}

fn as_integer(s: impl AsRef<str>) -> Option<i32> {
    s.as_ref().trim().parse().ok()
}
