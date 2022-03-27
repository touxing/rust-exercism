// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hash_magazine: HashMap<&str, u32> = HashMap::new();

    for &i in magazine {
        *hash_magazine.entry(i).or_insert(0) += 1;
    }

    for &i in note {
        let item = hash_magazine.entry(i).or_insert(0);
        if *item == 0 {
          return false;
        }
        *item -= 1;
    }

    true
}
