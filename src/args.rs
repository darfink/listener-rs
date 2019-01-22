/// Represents an event's arguments.
pub trait EventArgs<T> {
  /// Returns whether the propagation has been stopped or not.
  fn is_propagation_stopped(&self) -> bool;

  /// Returns whether the default action has been prevented or not.
  fn is_default_prevented(&self) -> bool;

  /// Stops the propagation of this event.
  fn stop_propagation(&mut self);

  /// Prevents the default action of this event.
  fn prevent_default(&mut self);

  /// Returns the data associated with this event.
  fn data(&self) -> &T;
}

pub struct EventArgsOwned<T> {
  stop_propagation: bool,
  prevent_default: bool,
  value: T,
}

impl<T> EventArgsOwned<T> {
  pub fn new(value: T) -> Self {
    EventArgsOwned {
      stop_propagation: false,
      prevent_default: false,
      value,
    }
  }
}

impl<T> EventArgs<T> for EventArgsOwned<T> {
  fn is_propagation_stopped(&self) -> bool {
    self.stop_propagation
  }

  fn is_default_prevented(&self) -> bool {
    self.prevent_default
  }

  fn stop_propagation(&mut self) {
    self.stop_propagation = true;
  }

  fn prevent_default(&mut self) {
    self.prevent_default = true;
  }

  fn data(&self) -> &T {
    &self.value
  }
}

pub struct EventArgsRef<'a, T: 'static> {
  stop_propagation: bool,
  prevent_default: bool,
  value: &'a T,
}

impl<'a, T> EventArgsRef<'a, T> {
  pub fn new(value: &'a T) -> Self {
    EventArgsRef {
      stop_propagation: false,
      prevent_default: false,
      value,
    }
  }
}

impl<'a, T> EventArgs<T> for EventArgsRef<'a, T> {
  fn is_propagation_stopped(&self) -> bool {
    self.stop_propagation
  }

  fn is_default_prevented(&self) -> bool {
    self.prevent_default
  }

  fn stop_propagation(&mut self) {
    self.stop_propagation = true;
  }

  fn prevent_default(&mut self) {
    self.prevent_default = true;
  }

  fn data(&self) -> &T {
    self.value
  }
}