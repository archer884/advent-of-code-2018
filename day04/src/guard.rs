use crate::event::{Event, EventKind};

#[derive(Debug)]
pub struct Guard {
    pub id: u16,
    events: Vec<Event>,
}

impl Guard {
    pub fn new(id: u16) -> Guard {
        Guard {
            id,
            events: Vec::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }

    pub fn time_asleep(&self) -> i32 {
        let mut slept_at = None;
        let mut slept = 0;

        for event in &self.events {
            match event.kind() {
                EventKind::Sleep => slept_at = Some(event.timestamp().minute),
                EventKind::Wake => match slept_at.take() {
                    None => (),
                    Some(time) => slept += (event.timestamp().minute - time) as i32,
                },

                _ => (), // Ignore shift start
            }
        }

        slept
    }

    pub fn sleep_report(&self) -> Vec<u8> {
        let mut slept_at = None;
        let mut slept = Vec::new();

        for event in &self.events {
            match event.kind() {
                EventKind::Sleep => slept_at = Some(event.timestamp().minute),
                EventKind::Wake => match slept_at.take() {
                    None => (),
                    Some(time) => slept.extend(time..event.timestamp().minute),
                },

                _ => (), // Ignore shift start
            }
        }

        slept
    }
}
