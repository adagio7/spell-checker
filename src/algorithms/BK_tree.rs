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

// #[cfg(test)]
// mod tests {
