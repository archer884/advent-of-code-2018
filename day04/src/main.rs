mod event;
mod guard;
mod timestamp;

use crate::{
    event::{Event, EventKind},
    guard::Guard,
};
use hashbrown::HashMap;
use std::error::Error;

// Parsing works. We get the right number of records, and the events are in order.
//
// Next step: Figure out how much time each guard spends asleep.
//
// Wrinkle: Guards often start work before midnight, which makes it harder to
// calculate the time they spend asleep. On the other hand...
//
// Mitigation: Guards never fall asleep before midnight.

fn main() -> Result<(), Box<Error>> {
    let (_, guard) = build_guards(read_events())
        .map(|x| (total_time_asleep(&x), x))
        .max_by_key(|x| x.0)
        .ok_or("Apparently, no one works.")?;

    let mut sleep_state = HashMap::new();
    for minute in guard.iter().flat_map(|x| x.sleep_report()) {
        *sleep_state.entry(minute).or_insert(0) += 1;
    }

    let guard_id = guard.first().ok_or("Seriously, man...")?.id;
    let (target_minute, _) = sleep_state
        .into_iter()
        .max_by_key(|x| x.1)
        .ok_or("Or maybe no one sleeps?")?;

    println!(
        "Guard ID ({}) * Target Minute ({}) = {}",
        guard_id,
        target_minute,
        i32::from(guard_id) * i32::from(target_minute),
    );
    Ok(())
}

fn total_time_asleep(guards: &[Guard]) -> i32 {
    guards.iter().map(|x| x.time_asleep()).sum()
}

fn build_guards(events: impl IntoIterator<Item = Event>) -> impl Iterator<Item = Vec<Guard>> {
    let mut guards = Vec::new();
    let mut guard = None;

    for event in events {
        match event.kind() {
            EventKind::Shift(id) => {
                if let Some(guard) = guard.take() {
                    guards.push(guard);
                }

                guard = Some(Guard::new(id));
            }

            _ => {
                if let Some(ref mut guard) = guard {
                    guard.push_event(event);
                }
            }
        }
    }

    let mut grouped = HashMap::new();
    for guard in guards {
        grouped.entry(guard.id).or_insert_with(Vec::new).push(guard);
    }

    grouped.into_iter().map(|(_, guard)| guard)
}

fn read_events() -> Vec<Event> {
    let mut events: Vec<_> = grabinput::from_stdin()
        .filter_map(|s| s.trim().parse::<Event>().ok())
        .collect();

    events.sort_by_key(|x| x.timestamp());
    events
}
