/*
Important Note:
  Breadth First Search does not preserve shape
  Depth First Search preserve shape!

  If the task relate to the shape of the structure consider using DFS!
*/

use crate::types::{BinaryNode};
use std::collections::VecDeque;

// Time:  O(N)
// Space: O(N)
pub fn bfs(head: BinaryNode<i32>, needle: i32) -> bool {
  let mut queue = VecDeque::<BinaryNode<i32>>::new();
  queue.push_back(head);

  loop {
    match queue.pop_front() {
      None => break,
      Some(node) => {
        if node.value == needle {
          return true;
        }
        
        if let Some(left) = node.left {
          queue.push_back(*left);
        }
        
        if let Some(right) = node.right {
          queue.push_back(*right);
        }
      }
    }
  }
  
  false
}


#[cfg(test)]
mod tests {
  use crate::tests::tree::{get_tree};
  use super::*;

  #[test]
  fn test() {
    assert_eq!(true, bfs(get_tree(), 45));
    assert_eq!(true, bfs(get_tree(), 7));
    assert_eq!(false, bfs(get_tree(), 69));
  }
}