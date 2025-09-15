use crate::ls::event::EventListener;

pub struct EventDispatcher<'i, Events>
where
    Events: Iterator<Item = &'i dyn EventListener>,
{
    events: Events,
}

impl<'i, Events> EventDispatcher<'i, Events>
where
    Events: Iterator<Item = &'i dyn EventListener>,
{
    pub fn new(events: Events) -> Self {
        Self { events }
    }
}
