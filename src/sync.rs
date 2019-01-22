//! Synchronized event handler implementation.

use std::sync::{Arc, RwLock};
use crate::{EventAction, EventListener};
use crate::args::{EventArgs, EventArgsOwned, EventArgsRef};

/// A synchronized event handler.
pub struct EventHandler<T: 'static> {
  listeners: Arc<RwLock<Vec<Box<dyn EventListener<T>>>>>,
}

impl<T: 'static> Clone for EventHandler<T> {
  fn clone(&self) -> Self {
    EventHandler {
      listeners: self.listeners.clone(),
    }
  }
}

impl<T: 'static> EventHandler<T> {
  /// Constructs a new event handler.
  pub fn new() -> Self {
    EventHandler {
      listeners: Arc::new(RwLock::new(Vec::new())),
    }
  }

  /// Dispatches an event with referenced data.
  pub fn dispatch_ref(&self, data: &T) -> bool {
    self.dispatch_impl(EventArgsRef::new(data))
  }

  /// Dispatches an event with owned data.
  pub fn dispatch(&self, data: T) -> bool {
    self.dispatch_impl(EventArgsOwned::new(data))
  }

  /// Subscribes a new listener to the event.
  pub fn subscribe<E: EventListener<T>>(&self, listener: E) {
    self.listeners.write().expect("poison").push(Box::new(listener));
  }

  /// Subscribes a new closure to the event.
  pub fn subscribe_fn<F, R>(&self, closure: F)
  where
    F: Fn(&mut EventArgs<T>) -> R + Send + 'static,
    R: Into<EventAction>,
  {
    self.subscribe(closure);
  }

  fn dispatch_impl(&self, mut event: impl EventArgs<T>) -> bool {
    let removed_listeners = self
      .listeners
      .read()
      .expect("poison")
      .iter()
      .enumerate()
      .rev()
      .filter_map(|(index, listener)| {
        if !event.is_propagation_stopped() {
          match listener.call(&mut event) {
            EventAction::Remove => Some(index),
            EventAction::Keep => None,
          }
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    if !removed_listeners.is_empty() {
      let mut listeners = self.listeners.write().expect("poison");
      for index in removed_listeners {
        listeners.remove(index);
      }
    }

    !event.is_default_prevented()
  }
}