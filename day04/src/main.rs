mod event;
mod timestamp;

use crate::event::Event;

// Parsing works. We get the right number of records, and the events are in order.
//
// Next step: Figure out how much time each guard spends asleep.
//
// Wrinkle: Guards often start work before midnight, which makes it harder to
// calculate the time they spend asleep. On the other hand...
//
// Mitigation: Guards never fall asleep before midnight.

fn main() {
    let mut events: Vec<_> = grabinput::from_stdin()
        .filter_map(|s| s.trim().parse::<Event>().ok())
        .collect();

    events.sort_by_key(|x| x.timestamp());

    for event in events {
        println!("{:?}", event);
    }
}
