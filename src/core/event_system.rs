use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Event: Any + Send + Sync {
    fn name(&self) -> &'static str;
}

pub type EventHandlerFn = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventSystem {
    handlers: HashMap<TypeId, Vec<EventHandlerFn>>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe<E: Event>(&mut self, handler: impl Fn(&E) + Send + Sync + 'static) {
        let type_id = TypeId::of::<E>();
        
        let handler_wrapper: EventHandlerFn = Box::new(move |event: &dyn Any| {
            if let Some(event) = event.downcast_ref::<E>() {
                handler(event);
            }
        });
        
        self.handlers.entry(type_id).or_insert_with(Vec::new).push(handler_wrapper);
    }

    pub fn publish<E: Event>(&self, event: E) {
        let type_id = TypeId::of::<E>();
        
        if let Some(handlers) = self.handlers.get(&type_id) {
            for handler in handlers {
                handler(&event);
            }
        }
    }

    pub fn clear<E: Event>(&mut self) {
        let type_id = TypeId::of::<E>();
        self.handlers.remove(&type_id);
    }

    pub fn clear_all(&mut self) {
        self.handlers.clear();
    }
} 