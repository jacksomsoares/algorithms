use std::rc::Rc;
use std::cell::{RefCell};

#[derive(Clone)]
pub struct Node<T: Clone> {
  pub value: T,
  pub next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct MyQueue<T: Clone> {
  head: Option<Rc<RefCell<Node<T>>>>,
  tail: Option<Rc<RefCell<Node<T>>>>,
  len: usize,
}

impl<T: Clone> MyQueue<T> {
  pub fn new() -> Self {
    Self {
      head: None,
      tail: None,
      len: 0,
    }
  }

  pub fn enqueue(&mut self, item: T) {
    let node = Node {
      value: item,
      next: None,
    };

    self.len += 1;
    match &mut self.tail {
      None => {
        let x = Rc::new(RefCell::new(node));
        self.head = Some(x.clone());
        self.tail = Some(x);
      }
      Some(tail) => {
        let x = Rc::new(RefCell::new(node));

        (*tail).borrow_mut().next = Some(x.clone());
        self.tail = Some(x);
      }
    }
  }

  pub fn dequeue(&mut self) -> Option<T> {
    let local_head;

    self.len -= 1;

    match &self.head {
      None => { return None; }
      Some(head) => {
        local_head = head.clone();
      }
    }

    match &RefCell::borrow(&*local_head).next {
      None => { self.head = None; }
      Some(next) => {
        self.head = Some(next.clone());
      }
    }

    let x = Some(RefCell::borrow(&*local_head).value.clone());
    x
  }

  pub fn len(self: &Self) -> usize {
    self.len
  } 
  
  pub fn peek(self: &Self) -> Option<T> {
    match &self.head {
      Some(head) => Some((**head).borrow().value.clone()),
      None => None
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut queue = MyQueue::<i32>::new();

    assert_eq!(None, queue.peek());
    queue.enqueue(2);
    assert_eq!(2, queue.peek().unwrap());
    queue.enqueue(3);
    queue.enqueue(4);
    assert_eq!(2, queue.peek().unwrap());
    assert_eq!(3, queue.len());
    
    assert_eq!(2, queue.dequeue().unwrap());
    assert_eq!(2, queue.len());
    
    assert_eq!(3, queue.dequeue().unwrap());
    assert_eq!(1, queue.len());
    
    assert_eq!(4, queue.dequeue().unwrap());
    assert_eq!(0, queue.len());

    assert_eq!(None, queue.peek());
  }
}