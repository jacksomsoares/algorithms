use std::collections::HashMap;
use std::hash::Hash;

struct Node<V> {
  value: V,
  next: usize,
  prev: usize,
}

pub struct LRU<K, V> {
  capacity: usize,
  list: Vec<Node<V>>,
  head: usize,
  tail: usize,
  lookup: HashMap<K, usize>,
  reverse_lookup: HashMap<usize, K>,
}

impl<K: Clone + PartialEq + Eq + Hash, V> LRU<K,V> {
  pub fn new(capacity: usize) -> Self {
    if capacity == usize::MAX {
      panic!("TODO: return handling... usize::MAX is reserved for this algorithm.");
    }

    Self {
      capacity: capacity,
      list: Vec::with_capacity(capacity),
      head: usize::MAX,
      tail: usize::MAX,
      lookup: HashMap::new(),
      reverse_lookup: HashMap::new(),
    }
  }

  pub fn update(&mut self, key: &K, value: V) {
    // does it exits?
    // if it doesn't we need to insert
    //   - Check capacity and evict if over

    // if it doest, we need to update to the front o the list and update the value
    match self.lookup.get(key) {
      None => {        
        let node = Node {
          value: value,
          next: self.head,
          prev: usize::MAX,
        };

        let slot;
        if self.list.len() >= self.capacity {
          
          //evict tail
          slot = self.tail;
          let tailPrev = self.list[slot].prev;
          self.tail = tailPrev;
          
          if tailPrev != usize::MAX {
            self.list[tailPrev].next = usize::MAX;
          }
          
          self.list[slot] = node;

          let key_to_evict = self.reverse_lookup.remove(&slot).unwrap();
          self.lookup.remove(&key_to_evict);
          
        } else {
          
          self.list.push(node);
          slot = self.list.len() - 1;
          if self.tail == usize::MAX {
            self.tail = slot;
          }
        }

        if self.head != usize::MAX {
          let headNode = &mut self.list[self.head];
          headNode.prev = slot;
        }

        self.head = slot;
        self.lookup.insert(key.clone(), slot);
        self.reverse_lookup.insert(slot, key.clone());
      },
      Some(val) => {
        //detach
        let val2 = *val;
        self.detach(val2);

        // Move to front
        self.list[val2].next = self.head;
        self.list[val2].prev = usize::MAX;
        self.list[val2].value = value;
      }
    }
  }

  pub fn get(&mut self, key: &K) -> Option<&V> {
    // check the cache for existence
    // return the value we found or None if not exists
    match self.lookup.get(key) {
      None => return None,
      Some(val) => {
        // update the value we found and move to the front
        let val2 = *val;
        if self.head == val2 {
          return Some(&self.list[val2].value);
        }

        if self.tail == val2 {
          self.tail = self.list[self.tail].prev;
        }

        //detach
        self.detach(val2);

        //Move to front
        let node = &mut self.list[val2];
        node.next = self.head;
        node.prev = usize::MAX;
        self.head = val2;

        return Some(&node.value);
      }
    }
  }

  fn detach(&mut self, node_to_detach: usize) {
    let node = &self.list[node_to_detach];
    let next = node.next;
    let prev = node.prev;
    
    if next != usize::MAX { 
      self.list[next].prev = prev;
    }
    
    if prev != usize::MAX { 
      self.list[prev].next = next;  
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test() {
    // LRU
    let mut lru = LRU::<String, i32>::new(3);

    assert_eq!(None, lru.get(&"foo".to_string()));

    lru.update(&"foo".to_string(), 69);
    assert_eq!(69, *lru.get(&"foo".to_string()).unwrap());

    lru.update(&"bar".to_string(), 420);
    assert_eq!(420, *lru.get(&"bar".to_string()).unwrap());

    lru.update(&"baz".to_string(), 1337);
    assert_eq!(1337, *lru.get(&"baz".to_string()).unwrap());

    lru.update(&"ball".to_string(), 69420);
    assert_eq!(69420, *lru.get(&"ball".to_string()).unwrap());
    assert_eq!(None, lru.get(&"foo".to_string()));
    assert_eq!(420, *lru.get(&"bar".to_string()).unwrap());

    lru.update(&"foo".to_string(), 69);
    assert_eq!(420, *lru.get(&"bar".to_string()).unwrap());
    assert_eq!(69, *lru.get(&"foo".to_string()).unwrap());

    // shouldn't of been deleted, but since bar was get'd, bar was added to the
    // front of the list, so baz became the end
    assert_eq!(None, lru.get(&"baz".to_string()));
  }
}
