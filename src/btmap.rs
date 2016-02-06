//! # Binary Tree Map
//!
//! An immutable map implemented with a binary tree.

use std::borrow::Borrow;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::rc::Rc;

struct Node<K, V> where K: Ord {
    key: K,
    val: V,
    left: Option<Rc<Node<K, V>>>,
    right: Option<Rc<Node<K, V>>>,
}

impl<K, V> Node<K, V> where K: Ord {
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where K: Ord + Borrow<Q>, Q: Ord
    {
        match key.cmp(self.key.borrow()) {
            Ordering::Equal => return Some(&self.val),
            Ordering::Less => if let Some(ref child) = self.left { return child.get(key) },
            Ordering::Greater => if let Some(ref child) = self.right { return child.get(key) },
        }
        None
    }
}

/// An immutable map implemented with a binary tree.
pub struct BtMap<K, V> where K : Ord {
    size: usize,
    root: Option<Rc<Node<K, V>>>
}

impl<K, V> BtMap<K, V> where K : Ord {
    /// Creates an empty map.
    pub fn empty() -> BtMap<K, V> {
        BtMap { size: 0, root: None }
    }

    /// Returns the number of items in the map.
    pub fn _len(&self) -> usize {
        self.size
    }

    /// Returns a reference to the item in the map corresponding to the key,
    /// or `None` if there is no item corresponding the the key.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where K: Ord + Borrow<Q>, Q: Ord
    {
        match self.root {
            Some(ref node) => node.get(key),
            None => None
        }
    }

    /// Returns `true` if the map contains an item corresponding to the key.
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
        where K: Ord + Borrow<Q>, Q: Ord
    {
        self.get(key).is_some()
    }

    /// Returns a new map containing all the items in the original map,
    /// as well as the new item.  Returns an error if there is already
    /// an item corresponding to the key.
    pub fn insert(&self, key: K, item: V) -> Result<BtMap<K, V>, ()> {
        match self.root {
            Some(ref node) => {
                match key.cmp(&node.key) {
                    Ordering::Equal => Err(()),
                    Ordering::Less => Ok(BtMap {
                        size: self.size + 1,
                        root: Some(Rc::new(Node {
                            key: key,
                            val: item,
                            left: None,
                            right: Some(node.clone()),
                        }))
                    }),
                    Ordering::Greater => Ok(BtMap {
                        size: self.size + 1,
                        root: Some(Rc::new(Node {
                            key: key,
                            val: item,
                            left: Some(node.clone()),
                            right: None,
                        }))
                    })
                }
            },
            None => {
                Ok(BtMap {
                    size: 1,
                    root: Some(Rc::new(Node {
                        key: key,
                        val: item,
                        left: None,
                        right: None,
                    }))
                })
            }
        }
    }
}

use std::clone::Clone;

impl<K, V> Clone for BtMap<K, V> where K: Ord {
    fn clone(&self) -> BtMap<K, V> {
        BtMap {
            size: self.size,
            root: match self.root {
                Some(ref root) => Some(root.clone()),
                None => None
            }
        }
    }

    fn clone_from(&mut self, source: &BtMap<K, V>) {
        self.size = source.size;
        self.root = match source.root {
            Some(ref root) => Some(root.clone()),
            None => None,
        }
    }
}

use std::ops::Index;

impl<'a, K, V, Q: ?Sized> Index<&'a Q> for BtMap<K, V>
    where K: Ord + Borrow<Q>, Q: Ord
{
    type Output = V;
    fn index(&self, index: &Q) -> &V {
        self.get(index).expect("no entry found for key")
    }
}

#[cfg(test)]
mod tests {
    use super::BtMap;

    #[test]
    fn test_empty() {
        let m : BtMap<i32, i32> = BtMap::empty();
        assert!(m._len() == 0);
        assert!(m.get(&34).is_none());
        assert!(!m.contains_key(&34));
    }

    #[test]
    fn test_nonempty() {
        let m : BtMap<usize, String> = BtMap::empty();
        let m = m.insert(1234, "OneTwoThreeFour".to_string()).unwrap();
        let m = m.insert(5543, "FiveFiveFourThree".to_string()).unwrap();
        let m = m.insert(8876, "EightEightSevenSix".to_string()).unwrap();
        let m = m.insert(22, "TwentyTwo".to_string()).unwrap();

        assert!(m._len() == 4);
        assert!(m.get(&22).unwrap() == "TwentyTwo");
        assert!(m.get(&5543).unwrap() == "FiveFiveFourThree");
        assert!(m.get(&3332).is_none());
        assert!(m.contains_key(&22));
        assert!(!m.contains_key(&111));
    }
}
