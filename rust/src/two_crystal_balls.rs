/**
Runtime: O(sqrt(n)).
space: O(1).
*/
pub fn two_crystal_balls_sqrt(breaks: &[bool]) -> isize {
  let jump_amout = (breaks.len() as f32).sqrt().floor() as usize;
  let len = breaks.len().try_into().unwrap();
  
  let mut i: usize = jump_amout;
  loop {
    if i >= len {
      break
    }
    match breaks[i] {
      true => break,
      false => i = i + jump_amout
    }
  }

  i = i - jump_amout;
  loop {  
    match breaks[i] {
      true => return i as isize,
      false => i += 1
    }
    
    if i >= len {
      break
    }
  }

  -1
}

/**
Runtime: O(log(n))
Space: O(1)
*/
pub fn two_crystal_balls_binary(breaks: &[bool]) -> isize {
  let mut lo = 0;
  let mut hi = breaks.len() - 1;

  let mut last_known_break: isize = -1;

  loop {
    if lo > hi {
      break
    }

    let mid = (lo + hi) / 2;

    if breaks[mid] {
      last_known_break = mid.try_into().unwrap();
      hi = match mid.checked_sub(1) {
        None => return last_known_break,
        Some(i) => i
      }
    } else {
      lo = mid + 1;
    }
  }
  
  last_known_break
}


#[cfg(test)]
mod tests {
  #[test]
  fn two_crystal_balls_tests_binary() {
    let balls1 = [true];
    let balls2 = [false];
    let balls3 = [false, true];
    let balls4 = [false, false];
    let balls5 = [ false, false, false, true, true, true ];
    let balls6 = [ false, false, false, false, false, false ];
    
    assert_eq!(0, super::two_crystal_balls_binary(&balls1));
    assert_eq!(-1, super::two_crystal_balls_binary(&balls2));
    assert_eq!(1, super::two_crystal_balls_binary(&balls3));
    assert_eq!(-1, super::two_crystal_balls_binary(&balls4));
    assert_eq!(3, super::two_crystal_balls_binary(&balls5));
    assert_eq!(-1, super::two_crystal_balls_binary(&balls6));
  }
  
  #[test]
  fn two_crystal_balls_tests_sqrt() {
    let balls1 = [ true ];
    let balls2 = [ false, true ];
    let balls3 = [ false, true, true ];
    let balls4 = [ false, false, true ];
    let balls5 = [ false, false, false, false, false, true ];
    let balls6 = [ false, false, false, true, true, true ];
    let balls7 = [ false, false, false, false, false, false ];
    
    assert_eq!(0, super::two_crystal_balls_sqrt(&balls1), "Test1");
    assert_eq!(1, super::two_crystal_balls_sqrt(&balls2), "Test2");
    assert_eq!(1, super::two_crystal_balls_sqrt(&balls3), "Test3");
    assert_eq!(2, super::two_crystal_balls_sqrt(&balls4), "Test4");
    assert_eq!(5, super::two_crystal_balls_sqrt(&balls5), "Test5");
    assert_eq!(3, super::two_crystal_balls_sqrt(&balls6), "Test6");
    assert_eq!(-1, super::two_crystal_balls_sqrt(&balls7), "Test7");
  }
}
