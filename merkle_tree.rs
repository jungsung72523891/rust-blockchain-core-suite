use std::collections::HashMap;
use crate::blockchain_crypto::BlockchainCrypto;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    leaves: Vec<String>,
    tree: Vec<Vec<String>>,
    proof_map: HashMap<String, Vec<String>>,
}

impl MerkleTree {
    pub fn new(transactions: &[String]) -> Self {
        let leaves: Vec<String> = transactions
            .iter()
            .map(|tx| BlockchainCrypto::sha256_hash(tx.as_bytes()))
            .collect();
        
        let mut tree = Self::build_tree(leaves.clone());
        let proof_map = Self::build_proof_map(&leaves, &tree);
        
        Self { leaves, tree, proof_map }
    }

    fn build_tree(mut leaves: Vec<String>) -> Vec<Vec<String>> {
        let mut tree = vec![leaves.clone()];
        
        while leaves.len() > 1 {
            let mut level = Vec::new();
            let mut i = 0;
            while i < leaves.len() {
                let left = &leaves[i];
                let right = if i + 1 < leaves.len() { &leaves[i+1] } else { left };
                let combined = format!("{}{}", left, right);
                let hash = BlockchainCrypto::sha256_hash(combined.as_bytes());
                level.push(hash);
                i += 2;
            }
            leaves = level;
            tree.push(leaves.clone());
        }
        tree
    }

    fn build_proof_map(leaves: &[String], tree: &[Vec<String>]) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        for (idx, leaf) in leaves.iter().enumerate() {
            let proof = Self::get_proof_by_index(idx, tree);
            map.insert(leaf.clone(), proof);
        }
        map
    }

    fn get_proof_by_index(index: usize, tree: &[Vec<String>]) -> Vec<String> {
        let mut proof = Vec::new();
        let mut idx = index;
        
        for level in 0..tree.len()-1 {
            let level_nodes = &tree[level];
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            
            if sibling_idx < level_nodes.len() {
                proof.push(level_nodes[sibling_idx].clone());
            }
            
            idx /= 2;
        }
        proof
    }

    pub fn root_hash(&self) -> String {
        if self.tree.is_empty() {
            return String::new();
        }
        self.tree.last().unwrap()[0].clone()
    }

    pub fn verify_proof(&self, leaf: &str, proof: &[String]) -> bool {
        let mut current = leaf.to_string();
        
        for sibling in proof {
            let combined = format!("{}{}", current, sibling);
            current = BlockchainCrypto::sha256_hash(combined.as_bytes());
        }
        current == self.root_hash()
    }
}
