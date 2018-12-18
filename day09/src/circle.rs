use std::cell::RefCell;

type Arena<'a, T> = typed_arena::Arena<RefCell<Node<'a, T>>>;
type Pointer<'a, T> = Option<*mut RefCell<Node<'a, T>>>;

// Each time you write a type definition like this one, God kills a whole litter of kittens.
pub struct Node<'a, T> {
    l: Pointer<'a, T>,
    r: Pointer<'a, T>,
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

    fn item(&self) -> &T {
        &self.item
    }
}

enum Shift {
    Left(usize),
    Right(usize),
}

pub struct Circle<'a> {
    arena: &'a Arena<'a, u32>,

    // It's raining dead kittens.
    current: Pointer<'a, u32>,
    lmost: Pointer<'a, u32>,
    rmost: Pointer<'a, u32>
}

impl<'a> Circle<'a> {
    pub fn new(arena: &'a mut Arena<'a, u32>) -> Circle<'a> {
        Circle {
            arena,
            current: None,
            lmost: None,
            rmost: None,
        }
    }

    pub fn push(&mut self, item: u32) {
        self.shift(Shift::Right(1));
        
        let append = self.arena.alloc(RefCell::new(Node::new(item)));

        // Forgive me, Father...
        if let Some(mut current) = self.current.as_ref().map(|&x| unsafe { &mut *x }.borrow_mut()) {
            if current.r.is_none() {
                self.rmost = Some(append);
            }
            append.borrow_mut().l = self.current;
            current.r = Some(append);
        } else {
            self.current = Some(append);
            self.lmost = Some(append);
            self.rmost = Some(append);
        }
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.shift(Shift::Left(7));

        // ...For I don't know what I'm doing.
        if let Some(current) = self.current.as_ref().map(|&x| unsafe { &*x }.borrow_mut()) {
            if current.l.is_none() {
                self.lmost = current.r;
            }
            
            if current.r.is_none() {
                self.rmost = current.l
            }

            self.current = current.r;

            return Some(*current.item())
        }
        None
    }

    fn shift(&mut self, shift: Shift) {
        if self.current.is_none() {
            return;
        }

        match shift {
            Shift::Left(places) => {
                for _ in 0..places {
                    match self.current.as_ref().and_then(|&x| unsafe { &*x }.borrow().l) {
                        Some(x) => self.current = Some(x),
                        None => self.current = self.rmost,
                    }
                }
            }

            Shift::Right(places) => {
                for _ in 0..places {
                    match self.current.as_ref().and_then(|&x| unsafe { &*x }.borrow().r) {
                        Some(x) => self.current = Some(x),
                        None => self.current = self.lmost,
                    }
                }
            }
        }
    }
}
