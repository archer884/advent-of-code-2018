use crate::step::Step;

struct Item {
    step: Step,
    turns_remaining: i32,
}

impl Item {
    fn new(step: Step) -> Item {
        Item {
            step,
            turns_remaining: step.turns(),
        }
    }
}

#[derive(Default)]
pub struct WorkQueue {
    items: Vec<Item>,
}

impl WorkQueue {
    pub(crate) fn new() -> WorkQueue {
        WorkQueue::default()
    }

    pub(crate) fn capacity(&self) -> usize {
        5 - self.items.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn increment<'a>(&'a mut self) -> impl Iterator<Item = Step> + 'a {
        self.items
            .iter_mut()
            .for_each(|item| item.turns_remaining -= 1);
        self.items
            .drain_filter(|item| item.turns_remaining == 0)
            .map(|item| item.step)
    }

    pub(crate) fn push(&mut self, step: Step) -> bool {
        if self.items.len() < 5 {
            self.items.push(Item::new(step));
            true
        } else {
            false
        }
    }
}
