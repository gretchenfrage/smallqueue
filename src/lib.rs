
extern crate through;

use through::*;

use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::fmt;

/// A FIFO queue which stores its data in-place when containing 0 or 1 elements, but
/// expands to a dynamically sized heap allocation when more elements are inserted, and
/// can free its heap allocation if the size returns later to 1 or 0.
pub struct SmallQueue<T> {
    state: SmallQueueState<T>
}
impl<T> SmallQueue<T> {
    /// New, empty queue.
    pub fn new() -> Self {
        SmallQueue {
            state: SmallQueueState::Zero
        }
    }

    /// New queue with a single element.
    pub fn of(elem: T) -> Self {
        SmallQueue {
            state: SmallQueueState::One(elem)
        }
    }

    /// Insert an element in insertion end.
    pub fn add(&mut self, elem: T) {
        through(&mut self.state, move |state| match state {
            SmallQueueState::Zero => SmallQueueState::One(elem),
            SmallQueueState::One(present) => {
                let mut queue = VecDeque::new();
                queue.push_back(present);
                queue.push_back(elem);
                SmallQueueState::Several(queue)
            },
            SmallQueueState::Several(mut queue) => {
                queue.push_back(elem);
                SmallQueueState::Several(queue)
            }
        })
    }

    /// Remove an element from the removal end.
    pub fn remove(&mut self) -> Option<T> {
        through_and(&mut self.state, |state| match state {
            SmallQueueState::Zero => (SmallQueueState::Zero, None),
            SmallQueueState::One(elem) => (SmallQueueState::Zero, Some(elem)),
            SmallQueueState::Several(mut queue) => {
                let elem = queue.pop_front();
                if queue.len() == 1 {
                    (SmallQueueState::One(queue.pop_front().unwrap()), elem)
                } else {
                    (SmallQueueState::Several(queue), elem)
                }
            }
        })
    }

    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        match &self.state {
            &SmallQueueState::Zero => true,
            _ => false,
        }
    }
}

enum SmallQueueState<T> {
    Zero,
    One(T),
    Several(VecDeque<T>)
}

impl<T: Debug> Debug for SmallQueue<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.state {
            SmallQueueState::Zero => f.write_str("[]"),
            SmallQueueState::One(ref elem) => {
                f.write_str("[")?;
                elem.fmt(f)?;
                f.write_str("]")
            },
            SmallQueueState::Several(ref queue) => queue.fmt(f),
        }
    }
}