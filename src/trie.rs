// src/trie.rs

use std::collections::HashMap;

/// Un noeud du trie.
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end: bool,
    pub name: Option<String>,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
            name: None,
        }
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

/// Le trie complet.
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Cree un trie vide.
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    /// Acces en lecture a la racine.
    pub fn root(&self) -> &TrieNode {
        &self.root
    }

    /// Insere un numero dans le trie.
    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = &mut self.root;

        for ch in number.chars() {
            current = current
                .children
                .entry(ch)
                .or_default();
        }

        current.is_end = true;
        current.name = Some(name.to_string());
    }

    /// Renvoie tous les numeros stockes, tries.
    pub fn collect_all(&self) -> Vec<String> {
        let mut results = Vec::new();

        Self::collect_recursive(&self.root, String::new(), &mut results);

        results
    }

    /// Parcours recursif en profondeur (DFS).
    fn collect_recursive(node: &TrieNode, prefix: String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(prefix.clone());
        }

        let mut keys: Vec<char> = node.children.keys().copied().collect();
        keys.sort();

        for ch in keys {
            if let Some(child) = node.children.get(&ch) {
                let mut p = prefix.clone();
                p.push(ch);

                Self::collect_recursive(child, p, results);
            }
        }
    }

    pub fn to_plantuml(&self) -> String {
        let mut result = String::new();

        result.push_str("@startmindmap\n");

        Self::build_puml(&self.root, 1, &mut result);

        result.push_str("@endmindmap\n");

        result
    }

    fn build_puml(node: &TrieNode, depth: usize, output: &mut String) {
        let mut keys: Vec<char> = node.children.keys().copied().collect();
        keys.sort();

        for ch in keys {
            if let Some(child) = node.children.get(&ch) {
                let stars = "*".repeat(depth);

                output.push_str(&format!("{} {}\n", stars, ch));

                Self::build_puml(child, depth + 1, output);

                if child.is_end && let Some(name) = &child.name {
                    let stars = "*".repeat(depth + 1);
                    output.push_str(&format!("{} {}\n", stars, name));
                }
            }
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
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

        trie.insert("04", "test");

        let n0 = trie.root().children.get(&'0').unwrap();
        let n4 = n0.children.get(&'4').unwrap();

        assert!(n4.is_end);
        assert!(!n0.is_end);
    }

    #[test]
    fn test_empty_trie_collect() {
        let trie = Trie::new();

        assert!(trie.collect_all().is_empty());
    }

    #[test]
    fn test_collect_single_number() {
        let mut trie = Trie::new();

        trie.insert("0612345678", "test");

        assert_eq!(trie.collect_all(), vec!["0612345678"]);
    }

    #[test]
    fn test_collect_shared_prefix() {
        let mut trie = Trie::new();

        trie.insert("042", "test1");
        trie.insert("043", "test2");
        trie.insert("041", "test3");

        assert_eq!(trie.collect_all(), vec!["041", "042", "043"]);
    }

    #[test]
    fn test_duplicate_insert() {
        let mut trie = Trie::new();

        trie.insert("0612", "test");
        trie.insert("0612", "test");

        assert_eq!(trie.collect_all().len(), 1);
    }
}