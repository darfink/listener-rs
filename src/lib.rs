#![deny(missing_docs)]
//! A library for event listeners.

pub use crate::args::EventArgs;
pub use crate::listener::EventListener;

mod args;
mod listener;
pub mod sync;

/// Represents an event action.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum EventAction {
  /// Removes the listener from the event.
  Remove,
  /// Keeps the listener in the event.
  Keep,
}

/// Creates a `Keep` action from a unit value.
impl From<()> for EventAction {
  fn from(_unit: ()) -> Self {
    EventAction::Keep
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let listener = sync::EventHandler::new();
    listener.subscribe_fn(|args| assert_eq!(*args.data(), 5));
    listener.dispatch(5);
  }
}