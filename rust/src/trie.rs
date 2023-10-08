const ALPHABET_SIZE: usize = 26;
const ZERO: usize = 'a' as usize;

#[derive(Debug)]
pub struct Trie {
  children: Box<[Option<Trie>; ALPHABET_SIZE]>,
  is_end_of_word: bool,
}

impl Trie {
  pub fn new() -> Self {
    const INIT: Option<Trie> = None;
    Self {
      children: Box::new([INIT; ALPHABET_SIZE]),
      is_end_of_word: false,
    } 
  }

  pub fn insert(&mut self, item: &str) {    
    let mut croot = self;    
    
    for c in item.as_bytes() {
      let index = *c as usize - ZERO;
      if croot.children[index].is_none() {
        croot.children[index] = Some(Trie::new())
      }

      croot = croot.children[index].as_mut().unwrap();
    }

    croot.is_end_of_word = true;
  }
  
  pub fn delete(&mut self, item: &str) {
    let str = item.as_bytes();
    if self.loope(str, 0) {
      let char = str[0] as usize - ZERO;
      self.children[char] = None;
    }
  }

  fn loope(&mut self, str: &[u8], idx: usize) -> bool {  
    let char = str[idx] as usize - ZERO;
    match &mut self.children[char] {
      None => todo!("Error handling"),
      Some(next) => {
        if idx == (str.len()-1) {
          next.is_end_of_word = false;
          if next.has_child() {
            return false;
          }
          return true;
        }
        if next.loope(str, idx+1) {
          if next.has_single_child() {
            return true;
          }
          return false;
        }
        return false;
      }
    }
  }

  fn has_child(&self) -> bool {
    for child in (self.children).iter() {
      match child {
        None => continue,
        Some(_) => return true,
      }
    }

    false
  }

  fn has_single_child(&self) -> bool {
    let mut count = 0;
    for child in (self.children).iter() {
      match child {
        None => continue,
        Some(_) => {
          count+=1;
          if count == 2 {
            return false;
          }
        },
      }
    }

    count == 1
  }

  pub fn find(&self, partial: &str) -> Vec<String> {
    let mut croot = self;
    let mut r = Vec::<String>::new();
    let mut string_builder = String::new();
    
    for c in partial.as_bytes() {
      let char_idx = *c as usize - ZERO;
      match &croot.children[char_idx] {
        None => return r,
        Some(next) => {
          string_builder.push(((char_idx + ZERO) as u8).into());
          if next.is_end_of_word {
            r.push(string_builder.clone());
            string_builder.clear();
          }
          croot = next;
        }
      }
    }

    let prefix = string_builder.clone();

    croot.depth_retrive_word(&mut string_builder, &mut r, prefix.as_str());

    r
  }

  fn depth_retrive_word(&self, string_builder: &mut String, r: &mut Vec<String>, prefix: &str) {    
    for (char_idx, char) in self.children.iter().enumerate() {
      match &char {
        None => continue,
        Some(next) => {
          string_builder.push(((char_idx + ZERO) as u8).into());
          if next.is_end_of_word {
            r.push(string_builder.clone());
          }
          next.depth_retrive_word(string_builder, r, prefix);
        }
      }
      string_builder.clear();
      string_builder.push_str(prefix);
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut trie = Trie::new();
    trie.insert("foo");
    trie.insert("fool");
    trie.insert("foolish");
    trie.insert("bar");
    
    let mut find = trie.find("fo");
    find.sort();
    assert_eq!(
      vec!["foo", "fool", "foolish"],
      find
    );
  
    trie.delete("fool");
  
    let mut find2 = trie.find("fo");
    find2.sort();
    assert_eq!(
      vec!["foo", "foolish"],
      find2
    );
  
    trie.delete("foo");
    trie.delete("foolish");
  
    println!("{trie:#?}"); //TODO: test to make sure the delete function is actually removing node from the tree instead of only setting the word to 'false'.
  }
}