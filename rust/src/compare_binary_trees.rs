use crate::types::BinaryNode;

fn compare_box(a: &Option<Box<BinaryNode<i32>>>, b: &Option<Box<BinaryNode<i32>>>) -> bool {
  match (a, b) {
    (None, None) => return true,
    (Some(_), None) | (None, Some(_)) => return false,
    (Some(a), Some(b)) => {
      if a.value != b.value {
        return false;
      }

      return compare_box(&a.left, &b.left) && compare_box(&a.right, &b.right);
    }
  }
}

pub fn compare(a: Option<&BinaryNode<i32>>, b: Option<&BinaryNode<i32>>) -> bool {

  match (a, b) {
    (None, None) => return true,
    (Some(_), None) | (None, Some(_)) => return false,
    (Some(a), Some(b)) => {
      if a.value != b.value {
        return false;
      }

      return compare_box(&a.left, &b.left) && compare_box(&a.right, &b.right);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test() {
    use crate::tests::tree::{get_tree, get_tree2};

    assert_eq!(true,  compare(Some(&get_tree()), Some(&get_tree ()))  );
    assert_eq!(false, compare(Some(&get_tree()), Some(&get_tree2()))  );
  }
}