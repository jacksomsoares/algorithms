pub struct BinaryNode<T> {
  pub value: T,
  pub left: Option<Box<BinaryNode<T>>>,
  pub right: Option<Box<BinaryNode<T>>>,
}

pub struct GraphEdge {
  pub to: usize,
  pub weight: f32,
}

pub type WeightedAdjacencyList = Vec<Vec<GraphEdge>>;
pub type WeightedAdjacencyMatrix = Vec<Vec<usize>>;
