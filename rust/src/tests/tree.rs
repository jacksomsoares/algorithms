use crate::types::BinaryNode;

pub fn get_tree() -> BinaryNode<i32> {
  BinaryNode::<i32> {
      value: 20,
      right: Some(Box::new( BinaryNode::<i32> {
          value: 50,
          right: Some(Box::new( BinaryNode::<i32> {
              value: 100,
              right: None,
              left: None,
          })),
          left: Some(Box::new( BinaryNode::<i32> {
              value: 30,
              right: Some(Box::new( BinaryNode::<i32> {
                  value: 45,
                  right: None,
                  left: None,
              })),
              left: Some(Box::new( BinaryNode::<i32> {
                  value: 29,
                  right: None,
                  left: None,
              }))
          })),
      })),
      left: Some(Box::new( BinaryNode::<i32>  {
          value: 10,
          right: Some(Box::new( BinaryNode::<i32>  {
              value: 15,
              right: None,
              left: None,
          })),
          left: Some(Box::new( BinaryNode::<i32> {
              value: 5,
              right: Some(Box::new( BinaryNode::<i32> {
                  value: 7,
                  right: None,
                  left: None,
              })),
              left: None,
          }))
      }))
  }
}


pub fn get_tree2() -> BinaryNode<i32> {
  BinaryNode::<i32> {
    value: 20,
    right: Some(Box::new( BinaryNode::<i32> {
        value: 50,
        right: None,
        left: Some(Box::new( BinaryNode::<i32> {
            value: 30,
            right: Some(Box::new( BinaryNode::<i32> {
                value: 45,
                right: Some(Box::new( BinaryNode::<i32> {
                    value: 49,
                    left: None,
                    right: None,
                })),
                left: None,
            })),
            left: Some(Box::new( BinaryNode::<i32> {
                value: 29,
                right: None,
                left: Some(Box::new( BinaryNode::<i32> {
                    value: 21,
                    right: None,
                    left: None,
                })),
            }))
        })),
    })),
    left: Some(Box::new( BinaryNode::<i32> {
        value: 10,
        right: Some(Box::new( BinaryNode::<i32> {
            value: 15,
            right: None,
            left: None,
        })),
        left: Some(Box::new( BinaryNode::<i32> {
            value: 5,
            right: Some(Box::new( BinaryNode::<i32> {
                value: 7,
                right: None,
                left: None,
            })),
            left: None,
        }))
    }))
  }
}
