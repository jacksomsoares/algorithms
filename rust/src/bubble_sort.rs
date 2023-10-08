/**
  Runtime: O(n^2)
  Space: O(1)
*/
pub fn bubble_sort(arr: &mut [isize]) -> &mut [isize] {
  for i in 0..arr.len() {
    for j in 0..arr.len() - 1 - i {
      if arr[j] > arr[j+1] {
        let temp = arr[j];
        arr[j] = arr[j+1];
        arr[j+1] = temp;
      }
    }
  }

  arr
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut arr1 = [10,9,8,7,6,5,4,3,2,1,0,-1];
    let mut arr2 = [-1,0,1,2,3,4,5,6,7,8,9,10];
    let mut arr3 = [3,2,7,10,8,7,100];

    assert_array_sorted(bubble_sort(&mut arr1));
    assert_array_sorted(bubble_sort(&mut arr2));
    assert_array_sorted(bubble_sort(&mut arr3));

    fn assert_array_sorted(arr: &[isize]) {
      let mut i = 0;
      let mut is_sorted = true;
      loop {
        if i >= (arr.len()-2) { break }
  
        if arr[i] <= arr[i+1] {
          i += 1;
          continue;
        }
  
        is_sorted = false;
        break;
      }
      
      assert_eq!(true, is_sorted);
    }
  }
}