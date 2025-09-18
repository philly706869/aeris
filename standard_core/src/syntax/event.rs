use std::collections::HashMap;

use uuid::Uuid;

pub struct EventManager {
    listeners: HashMap<EventListenerKey, Box<dyn EventListener>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<L>(&mut self, listener: L) -> EventListenerKey
    where
        L: EventListener + 'static,
    {
        let key = EventListenerKey(Uuid::new_v4());
        let listener = Box::new(listener);
        self.listeners.insert(key, listener);
        key
    }

    pub fn remove_listener(&mut self, key: EventListenerKey) -> bool {
        self.listeners.remove(&key).is_some()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventListenerKey(Uuid);

pub trait EventListener {
    fn on_update(&self);
}
