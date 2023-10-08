pub struct Stack<T> {
  storage: Vec<T>,
}

impl<T: Clone> Stack<T> {
  pub fn new() -> Self {
    Self {
      storage: Vec::new(),
    }
  }

  pub fn push(&mut self, item: T) {
    self.storage.push(item);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.storage.pop()
  }

  pub fn len(&self) -> usize {
    self.storage.len()
  }

  pub fn peek(&self) -> Option<T> {
    if self.storage.len() == 0 {
      return None;
    }
    
    Some(self.storage[self.storage.len() - 1].clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test() {
    let mut stack = Stack::<i32>::new();

    assert_eq!(0, stack.len());
    assert_eq!(None, stack.peek());
    
    stack.push(2);
    assert_eq!(1, stack.len());
    assert_eq!(Some(2), stack.peek());

    assert_eq!(Some(2), stack.pop());
    assert_eq!(0, stack.len());
    assert_eq!(None, stack.peek());

    stack.push(10);
    stack.push(11);
    stack.push(12);
    assert_eq!(3, stack.len());
    assert_eq!(Some(12), stack.peek());
    assert_eq!(Some(12), stack.pop());
    assert_eq!(Some(11), stack.pop());
    assert_eq!(Some(10), stack.pop());
    assert_eq!(None, stack.pop());
  }
}