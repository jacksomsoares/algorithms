use crate::types::{WeightedAdjacencyMatrix, WeightedAdjacencyList, GraphEdge};

//      (1) --- (4) ---- (5)
//    /  |       |       /|
// (0)   | ------|------- |
//    \  |/      |        |
//      (2) --- (3) ---- (6)
pub fn list1() -> WeightedAdjacencyList {
  vec![
    vec![
      GraphEdge { to: 1, weight: 3.0 },
      GraphEdge { to: 2, weight: 1.0 }
    ],
    vec![
      GraphEdge { to: 0, weight: 3.0 },
      GraphEdge { to: 2, weight: 4.0 },
      GraphEdge { to: 4, weight: 1.0 },
    ],
    vec![
      GraphEdge { to: 1, weight: 4.0 },
      GraphEdge { to: 3, weight: 7.0 },
      GraphEdge { to: 0, weight: 1.0 },
    ],
    vec![
      GraphEdge { to: 2, weight: 7.0 },
      GraphEdge { to: 4, weight: 5.0 },
      GraphEdge { to: 6, weight: 1.0 },
    ],
    vec![
      GraphEdge { to: 1, weight: 1.0 },
      GraphEdge { to: 3, weight: 5.0 },
      GraphEdge { to: 5, weight: 2.0 },
    ],
    vec![
      GraphEdge { to: 6, weight: 1.0 },
      GraphEdge { to: 4, weight: 2.0 },
      GraphEdge { to: 2, weight: 18.0 },
    ],
    vec![
      GraphEdge { to: 3, weight: 1.0 },
      GraphEdge { to: 5, weight: 1.0 },
    ],
  ]
}

//     >(1)<--->(4) ---->(5)
//    /          |       /|
// (0)     ------|------- |
//    \   v      v        v
//     >(2) --> (3) <----(6)
pub fn list2() -> WeightedAdjacencyList {
  vec![
    vec![
      GraphEdge { to: 1, weight: 3.0 },
      GraphEdge { to: 2, weight: 1.0 },
    ],
    vec![
      GraphEdge { to: 4, weight: 1.0 },
    ],
    vec![
      GraphEdge { to: 3, weight: 7.0 },
    ],
    vec![ ],
    vec![
      GraphEdge { to: 1, weight: 1.0 },
      GraphEdge { to: 3, weight: 5.0 },
      GraphEdge { to: 5, weight: 2.0 },
    ],
    vec![
      GraphEdge { to: 2, weight: 18.0 },
      GraphEdge { to: 6, weight: 1.0 },
    ],
    vec![
      GraphEdge { to: 3, weight: 1.0 },
    ],
  ]
}

//     >(1)<--->(4) ---->(5)
//    /          |       /|
// (0)     ------|------- |
//    \   v      v        v
//     >(2) --> (3) <----(6)
pub fn matrix2() -> WeightedAdjacencyMatrix {
  vec![
    vec![0, 3, 1,  0, 0, 0, 0],
    vec![0, 0, 0,  0, 1, 0, 0],
    vec![0, 0, 7,  0, 0, 0, 0],
    vec![0, 0, 0,  0, 0, 0, 0],
    vec![0, 1, 0,  5, 0, 2, 0],
    vec![0, 0, 18, 0, 0, 0, 1],
    vec![0, 0, 0,  1, 0, 0, 1],
  ]
}