use std::collections::VecDeque;

pub trait GameEvent {}

#[derive(Debug, Clone)]
pub struct Event<T: GameEvent> {
    pub event: T,
    pub timestamp: f64,
}

pub struct EventQueue<T: GameEvent> {
    events: VecDeque<Event<T>>,
}

impl<T: GameEvent> EventQueue<T> {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: T, timestamp: f64) {
        self.events.push_back(Event { event, timestamp });
    }

    pub fn pop(&mut self) -> Option<Event<T>> {
        self.events.pop_front()
    }

    pub fn drain(&mut self) -> Vec<Event<T>> {
        self.events.drain(..).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
