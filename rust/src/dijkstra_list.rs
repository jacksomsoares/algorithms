use crate::types::WeightedAdjacencyList;

fn has_unvisited(seen: &[bool], dists: &[f32]) -> bool {
  seen.iter().enumerate().any(|(i, s)| !s && dists[i] < f32::INFINITY)
}

fn get_lowest_unvisited(seen: &[bool], dists: &[f32]) -> usize {
  let mut idx = usize::MAX;
  let mut lowest_distance = f32::INFINITY;
  
  for (i, s) in seen.iter().enumerate() {
    if *s {
      continue;
    }

    if lowest_distance > dists[i] {
      lowest_distance = dists[i];
      idx = i;
    }
  }

  idx
}

pub fn dijkstra_list(source: usize, sink: usize, arr: &WeightedAdjacencyList) -> Vec<usize> {
  let mut seen = vec![false; arr.len()];
  let mut dists = vec![f32::INFINITY; arr.len()];
  let mut prev = vec![usize::MAX; arr.len()];
  dists[source] = 0.0;

  while has_unvisited(&seen, &dists) {
    let curr = get_lowest_unvisited(&seen, &dists);
    seen[curr] = true;

    for edge in &arr[curr] {
      if seen[edge.to] {
        continue;
      }

      let dist = dists[curr] + edge.weight;
      if dist < dists[edge.to] {
        dists[edge.to] = dist;
        prev[edge.to] = curr;
      }
    }
  }

  let mut curr = sink;
  let mut out = vec![];

  while prev[curr] != usize::MAX {
    out.push(curr);
    curr = prev[curr];
  }

  out.push(source);
  out.reverse();
  out
}

#[cfg(test)]
mod tests {
  use crate::tests::graph::{list1};
  use super::*;
  
  #[test]
  fn test() {
    // dijkstra via adj list
    assert_eq!(
      vec![0, 1, 4, 5, 6],
      dijkstra_list(0, 6, &list1())
    );
  }
}