use crate::types::WeightedAdjacencyList;

fn walk(graph: &WeightedAdjacencyList, curr: usize, needle: usize, seen: &mut [bool], path: &mut Vec<usize>) -> bool {
  if seen[curr] {
    return false;
  }
  seen[curr] = true;
  
  path.push(curr);
  if curr == needle {
    return true;
  }

  for edge in &graph[curr] {
    if walk(graph, edge.to, needle, seen, path) {
      return true;
    }
  }

  path.pop();
  false
}

pub fn dfs(graph: &WeightedAdjacencyList, source: usize, needle: usize) -> Option<Vec<usize>> {

  let mut seen = vec![false;graph.len()];
  let mut path = Vec::new();
  
  if walk(graph, source, needle, &mut seen, &mut path) {
    return Some(path);
  }

  None
}

#[cfg(test)]
mod tests {
  use crate::tests::graph::list2;
  use super::*;
  #[test]
  fn test() {
    // dfs - graph
    assert_eq!(
      vec![0, 1, 4, 5, 6],
      dfs(&list2(), 0, 6).unwrap()
    );
  
    assert_eq!(None, dfs(&list2(), 6, 0));
  }
}