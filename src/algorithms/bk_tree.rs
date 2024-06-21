use std::collections::{ HashSet, HashMap};

use crate::algorithms::base::SpellChecker;

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

pub struct BKTree {
    root: Option<Node>,
    spell_checker: Box<dyn SpellChecker>,
}

impl BKTree {
    pub fn new(spell_checker: Box<dyn SpellChecker>) -> BKTree {
        BKTree {
            root: None,
            spell_checker,
        }
    }

    fn add(&mut self, word: &String) -> () {
        // Initialize the root node if it doesn't exist
        if self.root.is_none() {
            self.root = Some(Node::new(word.to_string()));
            return;
        }

        // Note that we already checked that the root is not None
        let mut curr: &mut Node = self.root.as_mut().unwrap();

        loop {
            let dist = self.spell_checker.distance(&curr.word, word);

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

    pub fn load_dictionary(&mut self, dictionary: &HashSet<String>) -> () {
        for word in dictionary.iter() {
            self.add(word);
        }
    }


    pub fn search(&self, word: &str, max_distance: usize) -> Vec<String> {
        let mut results = vec![];

        if self.root.is_none() {
            panic!("The BK tree is empty, populate it first");
        }

        let mut stack = vec![self.root.as_ref().unwrap()];
        while let Some(node) = stack.pop() {
            let dist = self.spell_checker.distance(&node.word, word);

            if dist <= max_distance {
                results.push(node.word.clone());
            }

            let start = dist.checked_sub(max_distance).unwrap_or(0);
            let end = dist + max_distance;

            for i in start..=end {
                if let Some(child) = node.get_child(i) {
                    stack.push(child);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::levenshtein::Levenshtein;

    #[test]
    fn test_add_root() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);
        tree.add(&"hello".to_string());

        assert_eq!(tree.root.is_some(), true);
        assert_eq!(tree.root.as_ref().unwrap().word, "hello");
    }

    #[test]
    fn test_add_child() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);
        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());

        let root = tree.root.as_ref().unwrap();
        let child = root.get_child(1).unwrap();

        assert_eq!(child.word, "hella");
    }

    #[test]
    fn test_add_grandchild() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

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
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hello".to_string());

        let root = tree.root.as_ref().unwrap();

        assert_eq!(root.children.len(), 0);
    }

    #[test]
    fn test_search_max_dist_one() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let mut results = tree.search(&"hello".to_string(), 1);
        let mut expected = vec!["hello", "hella", "hallo"];

        results.sort();
        expected.sort();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_no_results() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let mut results = tree.search(&"world".to_string(), 1);
        let mut expected = vec![] as Vec<String>;

        results.sort();
        expected.sort();
        assert_eq!(results, expected);
    }

    #[test]
    #[should_panic]
    fn test_search_empty_tree() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let tree = BKTree::new(spell_checker);

        tree.search(&"hello".to_string(), 1);
    }

    #[test]
    fn test_search_max_dist_two() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());
        tree.add(&"halo".to_string());

        let mut results = tree.search(&"hello".to_string(), 2);
        let mut expected = vec!["hello", "hella", "hallo", "halo"];

        results.sort();
        expected.sort();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_exact_match_present() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let mut results = tree.search(&"hello".to_string(), 0);
        let mut expected = vec!["hello"];

        results.sort();
        expected.sort();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_non_present_exact_match() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let results = tree.search(&"world".to_string(), 0);
        let expected = vec![] as Vec<String>;

        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_case_sensitive() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let results = tree.search(&"HELLO".to_string(), 0);
        let expected = vec![] as Vec<String>;

        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_no_match() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let results = tree.search(&"world".to_string(), 2);

        assert_eq!(results, vec![] as Vec<String>);
    }

    #[test]
    fn test_search_empty_string() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let results = tree.search(&"".to_string(), 4);
        let expected = vec![] as Vec<String>;

        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_match_prefix() {
        let spell_checker = Box::new(Levenshtein::new(1));
        let mut tree = BKTree::new(spell_checker);

        tree.add(&"hello".to_string());
        tree.add(&"hella".to_string());
        tree.add(&"hallo".to_string());

        let mut results = tree.search(&"hell".to_string(), 1);
        let mut expected = vec!["hello", "hella"];

        results.sort();
        expected.sort();
        assert_eq!(results, expected);
    }
}
