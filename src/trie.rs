use std::collections::HashMap;

pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn root(&self) -> &TrieNode {
        &self.root
    }

    pub fn insert(&mut self, number: &str) {
        let mut current = &mut self.root;
        for ch in number.chars() {
            current = current.children
                .entry(ch)
                .or_insert_with(TrieNode::new);
        }
        current.is_end = true;
    }

    pub fn collect_all(&self) -> Vec<String> {
        let mut results = Vec::new();
        Self::collect_recursive(
            &self.root,
            String::new(),
            &mut results
        );
        results
    }

    fn collect_recursive(
        node: &TrieNode,
        prefix: String,
        results: &mut Vec<String>
    ) {
        if node.is_end {
            results.push(prefix.clone());
        }
        let mut keys: Vec<char> =
            node.children.keys().copied().collect();
        keys.sort();
        for ch in keys {
            if let Some(child) = node.children.get(&ch) {
                let mut new_prefix = prefix.clone();
                new_prefix.push(ch);
                Self::collect_recursive(
                    child, new_prefix, results
                );
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_new_is_empty() {
        let trie = Trie::new();
        assert!(trie.root().children.is_empty());
        assert!(!trie.root().is_end);
    }

    #[test]
    fn test_insert_creates_nodes() {
        let mut trie = Trie::new();
        trie.insert("04");
        let node_0 = trie.root().children.get(&'0')
            .expect("noeud 0 absent");
        let node_4 = node_0.children.get(&'4')
            .expect("noeud 4 absent");
        assert!(node_4.is_end);
        assert!(!node_0.is_end);
    }

    #[test]
    fn test_empty_trie_collect() {
        let trie = Trie::new();
        let results = trie.collect_all();
        assert!(results.is_empty());
    }

    #[test]
    fn test_collect_single_number() {
        let mut trie = Trie::new();
        trie.insert("0612345678");
        let results = trie.collect_all();
        assert_eq!(results, vec!["0612345678"]);
    }


}
