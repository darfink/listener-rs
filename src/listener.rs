use crate::{EventArgs, EventAction};

/// A trait describing interfaces that can listen for an event.
pub trait EventListener<T>: Send + 'static {
  /// Calls the listener with associated event arguments.
  fn call(&self, event: &mut EventArgs<T>) -> EventAction;
}

/// Closures can act as event listeners.
impl<F, T, R> EventListener<T> for F
where
  F: Fn(&mut EventArgs<T>) -> R + Send + 'static,
  R: Into<EventAction>,
{
  fn call(&self, event: &mut EventArgs<T>) -> EventAction {
    self(event).into()
  }
}
