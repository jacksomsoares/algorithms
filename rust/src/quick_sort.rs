pub fn quick_sort(arr: &mut [i32]) {
  qs(arr, 0, arr.len() - 1);
}

fn qs(arr: &mut[i32], lo: usize, hi: usize) {
  if lo >= hi {
    return;
  }
  
  let p = partition(arr, lo, hi);
  qs(arr, lo, p-1);
  qs(arr, p+1, hi);
}

fn partition(arr: &mut[i32], lo: usize, hi: usize) -> usize {
  let pivot = arr[hi];
  let mut swap_idx = lo;

  for i in lo..hi {
    if arr[i] <= pivot {
      let tmp = arr[swap_idx];
      arr[swap_idx] = arr[i];
      arr[i] = tmp;
      swap_idx += 1;
    }
  }

  arr[hi] = arr[swap_idx];
  arr[swap_idx] = pivot;
  swap_idx
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut arr = [9, 3, 7, 4, 69, 420, 42];

    quick_sort(&mut arr);

    assert_eq!([3, 4, 7, 9, 42, 69, 420], arr);
  }
}