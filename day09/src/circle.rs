use std::cell::RefCell;

type Arena<'a, T> = typed_arena::Arena<RefCell<Node<'a, T>>>;

// Each time you write a type definition like this one, God kills a whole litter of kittens.
pub struct Node<'a, T> {
    l: Option<&'a RefCell<Node<'a, T>>>,
    r: Option<&'a RefCell<Node<'a, T>>>,
    item: T,
}

impl<'a, T> Node<'a, T> {
    fn new(item: T) -> Node<'a, T> {
        Node {
            l: None,
            r: None,
            item,
        }
    }

    fn append_after_self(&mut self, item: T, arena: &'a mut Arena<'a, T>) {
        let append = arena.alloc(RefCell::new(Node::new(item)));

        if let Some(mut r) = self.r.as_ref().map(|x| x.borrow_mut()) {
            append.borrow_mut().l = r.l.take();
            r.l = Some(append);
        }

        self.r = Some(append);
    }
}

enum Shift {
    Left(usize),
    Right(usize),
}

pub struct Circle<'a, T: 'static> {
    arena: &'a Arena<'static, T>,

    // It's raining dead kittens.
    current: Option<&'a RefCell<Node<'a, T>>>,
    leftmost: Option<&'a RefCell<Node<'a, T>>>,
    rightmost: Option<&'a RefCell<Node<'a, T>>>,
}

impl<'a, T> Circle<'a, T> {
    pub fn new(arena: &'a mut Arena<'static, T>) -> Circle<'a, T> {
        Circle {
            arena,
            current: None,
        }
    }

    pub fn push(&mut self, item: T) {
        self.shift(Shift::Right(1));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.shift(Shift::Left(7));

        unimplemented!()
    }

    fn shift(&mut self, shift: Shift) {
        match shift {
            Shift::Left(n) => {
                for _ in 0..n {
                    if let Some(left) = self.current.
                }
            }
        }
    }
}
