pub struct MinHeap {
  data: Vec<i32>,
}

impl MinHeap {
  pub fn new() -> Self {
    Self {
      data: Vec::<i32>::new(),
    }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn insert(&mut self, value: i32) {
    let len = self.data.len();
    self.data.push(value);
    self.heapify_up(len);
  }
  
  pub fn delete(&mut self) -> Option<i32> {
    match self.data.pop() {
      None => None,
      Some(max) => {
        if self.data.is_empty() {
          return Some(max);
        }

        let out = self.data[Self::root_idx()];
        self.data[Self::root_idx()] = max;
        self.heapify_down(Self::root_idx());
        return Some(out);        
      }
    }
  }

  fn root_idx() -> usize { 0 }

  fn heapify_down(&mut self, idx: usize) {
    let ridx = Self::right_child(idx);
    let lidx = Self::left_child(idx);

    if ridx >= self.data.len() || lidx >= self.data.len() {
      return;
    }

    let rv = self.data[ridx];
    let lv = self.data[lidx];
    let v = self.data[idx];

    if lv > rv && v > rv {
      self.data[ridx] = v;
      self.data[idx] = rv;
      self.heapify_down(ridx);
    } else if rv > lv && v > lv {
      self.data[lidx] = v;
      self.data[idx] = lv;
      self.heapify_down(lidx);
    }
  }

  fn heapify_up(&mut self, idx: usize) {
    if idx == 0 {
      return;
    }

    let p = Self::parent(idx);
    let parent_v = self.data[p];
    let v = self.data[idx];

    if parent_v > v {
      self.data[p] = v;
      self.data[idx] = parent_v;
      self.heapify_up(p);
    }
  }

  fn parent(idx: usize) -> usize {
    (idx-1)/2
  }

  fn left_child(idx: usize) -> usize {
    idx*2+1
  }

  fn right_child(idx: usize) -> usize {
    idx*2+2
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut heap = MinHeap::new();
  
    assert_eq!(0, heap.len());
  
    heap.insert(5);
    heap.insert(3);
    heap.insert(69);
    heap.insert(420);
    heap.insert(4);
    heap.insert(1);
    heap.insert(8);
    heap.insert(7);
  
    assert_eq!(8, heap.len());
  
    assert_eq!(1, heap.delete().unwrap());
    assert_eq!(3, heap.delete().unwrap());
    assert_eq!(4, heap.delete().unwrap());
    assert_eq!(5, heap.delete().unwrap());
    assert_eq!(4, heap.len());
    assert_eq!(7, heap.delete().unwrap());
    assert_eq!(8, heap.delete().unwrap());
    assert_eq!(69, heap.delete().unwrap());
    assert_eq!(420, heap.delete().unwrap());
    assert_eq!(0, heap.len());
  }
}