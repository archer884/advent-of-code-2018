#![feature(drain_filter)] // Cool!

mod queue;
mod step;

use crate::{queue::WorkQueue, step::Step};
use hashbrown::{HashMap, HashSet};

fn main() {
    let mut steps = dependencies_by_step(grabinput::from_stdin());
    let mut completed_steps = HashSet::new();

    let mut queue = WorkQueue::new();
    while !steps.is_empty() || !queue.is_empty() {
        let mut eligible = eligible_steps(&steps, &completed_steps);
        eligible.sort();

        for item in eligible.into_iter().take(queue.capacity()) {
            queue.push(item);
            steps.remove(&item);
        }

        for item in queue.increment() {
            completed_steps.insert(item);
        }
    }

    println!("{}", queue.seconds());
}

fn eligible_steps(steps: &HashMap<Step, Vec<Step>>, complete: &HashSet<Step>) -> Vec<Step> {
    steps
        .iter()
        .filter(|kv| kv.1.is_empty() || kv.1.iter().all(|step| complete.contains(step)))
        .map(|(&step, _)| step)
        .collect()
}

fn dependencies_by_step(input: impl IntoIterator<Item = String>) -> HashMap<Step, Vec<Step>> {
    let mut map = HashMap::new();
    for pair in input.into_iter().filter_map(parse_tuple) {
        map.entry(pair.0).or_insert_with(Vec::new);
        map.entry(pair.1).or_insert_with(Vec::new).push(pair.0);
    }
    map
}

fn parse_tuple(s: impl AsRef<str>) -> Option<(Step, Step)> {
    let s = s.as_ref().as_bytes();
    Some((Step(*s.get(5)?), Step(*s.get(36)?)))
}
