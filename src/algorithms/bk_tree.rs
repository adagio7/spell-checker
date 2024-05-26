use std::collections::HashMap;

use crate::algorithms::base::SpellChecker;
use crate::algorithms::levenshtein::Levenshtein;

struct Node {
    word: String,
    children: HashMap<usize, Node>,
}

impl Node {
    fn new(word: String) -> Node {
        Node {
            word,
            children: HashMap::new(),
        }
    }

    fn add_child(&mut self, distance: usize, node: Node) {
        self.children.insert(distance, node);
    }

    fn get_child(&self, distance: usize) -> Option<&Node> {
        self.children.get(&distance)
    }

    fn get_mut_child(&mut self, distance: usize) -> Option<&mut Node> {
        self.children.get_mut(&distance)
    }
}

struct BKTree {
    root: Option<Node> 
}

impl BKTree {
    fn new() -> BKTree {
        BKTree { root: None }
    }

    fn add(&mut self, word: &String) -> () {
        // Initialize the root node if it doesn't exist
        if self.root.is_none() {
            self.root = Some(Node::new(word.to_string()));
            return;
        }

        // Note that we already checked that the root is not None
        let mut curr: &mut Node = self.root.as_mut().unwrap();
        let spell_checker = Levenshtein::new(1);

        loop {
            let dist = spell_checker.distance(&curr.word, word);

            // If the distance is 0, the word is already in the tree
            if dist == 0 {
                return;
            }

            if curr.get_mut_child(dist).is_none() {
                curr.add_child(dist, Node::new(word.to_string()));
                return;
            } else {
                curr = curr.get_mut_child(dist).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_root() {
        let mut tree = BKTree::new();
        tree.add(&"hello".to_string());

        assert_eq!(tree.root.is_some(), true);
        assert_eq!(tree.root.as_ref().unwrap().word, "hello");
    }

    #[test]
    fn test_add_child() {
        let mut tree = BKTree::new();
        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());

        let root = tree.root.as_ref().unwrap();
        let child = root.get_child(1).unwrap();

        assert_eq!(child.word, "hella");
    }

    #[test]
    fn test_add_grandchild() {
        let mut tree = BKTree::new();
        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let root = tree.root.as_ref().unwrap();
        let child = root.get_child(1).unwrap();
        let grandchild = child.get_child(2).unwrap();

        assert_eq!(grandchild.word, "hallo");
    }

    #[test]
    fn test_add_duplicate() {
        let mut tree = BKTree::new();
        tree.add(&"hello".to_string());
        tree.add(&"hello".to_string());

        let root = tree.root.as_ref().unwrap();

        assert_eq!(root.children.len(), 0);
    }
}
