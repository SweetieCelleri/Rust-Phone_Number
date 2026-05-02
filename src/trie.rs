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
