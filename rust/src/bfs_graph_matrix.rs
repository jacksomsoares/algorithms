use crate::types::*;
use std::collections::VecDeque;

pub fn bfs(graph: &WeightedAdjacencyMatrix, source: usize, needle: usize) -> Option<Vec<usize>> {
  let mut seen = vec![false; graph.len()];
  let mut prev = vec![usize::MAX; graph.len()];

  let mut q = VecDeque::new();

  seen[source] = true;
  let mut curr = source;
  loop {
    if curr == needle {
      break;
    }

    let adjs = &graph[curr];
    for i in 0..adjs.len() {
      if adjs[i] == 0 {
        continue;
      }

      if seen[i] {
        continue;
      }
      
      seen[i] = true;
      prev[i] = curr;
      q.push_back(i);
    }

    println!("{q:?}");
    if let Some(next) = q.pop_front() {
      curr = next;
    } else {
      break;
    }
  }

  println!("{prev:?}");

  if prev[needle] == usize::MAX {
    return None;
  }

  curr = needle;
  let mut out = Vec::new();

  loop {
    if prev[curr] == usize::MAX {
      break;
    }
    out.push(curr);
    curr = prev[curr];
  }

  out.push(source);
  out.reverse();

  Some(out)
}

#[cfg(test)]
mod tests {
  use crate::tests::graph::{matrix2};
  use super::*;
  #[test]
  fn test() {
    // "bfs - graph matrix"  
    assert_eq!(
      vec![0, 1, 4, 5, 6],
      bfs(&matrix2(), 0, 6).unwrap()
    );
  
    assert_eq!(None, bfs(&matrix2(), 6, 0));
  }
}