use std::collections::HashSet;
use std::cmp::Ordering;
use std::fmt::{self, Debug};

#[derive(Eq, PartialEq)]
struct Set {
    elements: HashSet<u32>,
}

impl Set {
    fn new() -> Self {
        Set {
            elements: HashSet::new(),
        }
    }

    fn from_slice(slice: &[u32]) -> Self {
        Set {
            elements: slice.iter().copied().collect(),
        }
    }

    fn union(&self, other: &Set) -> Set {
        let mut new_set = self.elements.clone();
        new_set.extend(&other.elements);
        Set { elements: new_set }
    }
}

// Debug implementacja, aby można było wypisać Set
impl Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for elem in &self.elements {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
            first = false;
        }
        write!(f, "}}")
    }
}

// Porównanie zbiorów pod względem zawierania
impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.elements == other.elements {
            Some(Ordering::Equal)
        } else if self.elements.is_subset(&other.elements) {
            Some(Ordering::Less)
        } else if self.elements.is_superset(&other.elements) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

fn main() {
    println!("Set test:");
    let a = Set::from_slice(&[1, 2, 3]);
    let b = Set::from_slice(&[1, 2, 3, 4]);
    let c = a.union(&b);
    println!("a < b: {}", a < b);
    println!("Union of a and b: {:?}", c);
}
