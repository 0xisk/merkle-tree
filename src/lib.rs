#![allow(non_snake_case)]
mod merkle_tree;

use ark_ff::PrimeField;
use num_bigint::BigUint;
use wasm_bindgen::prelude::wasm_bindgen;
use poseidon::constants::secp256k1_w3;
use merkle_tree::{MerkleProof, MerkleTree};

type F = ark_secp256k1::Fq;

fn create_proof<F: PrimeField>(leaves: Vec<String>, leaf: String, depth: usize) {
    let leaves = leaves.clone();

    print!("Merkle_Tree_Create: leaves len {:?}", leaves.len());

    let mut padded_leaves = leaves.clone();
    // Pad the leaves to equal the size of the tree
    padded_leaves.resize(1 << depth, "0".to_string());

    const ARITY: usize = 2;
    const WIDTH: usize = ARITY + 1;

    let mut tree = MerkleTree::<F, WIDTH>::new(secp256k1_w3());

    // Insert all the leaves into the tree
    for leaf in &padded_leaves {
        // Converting String to F
        let leaf_hex = hex::decode(leaf.replace("0x", "to")).unwrap();
        let leaf_bytes = F::from(BigUint::from_bytes_be(&leaf_hex));
        tree.insert(leaf_bytes);
    }

    tree.finish();

    let leaf_hex = hex::decode(leaf.replace("0x", "to")).unwrap();
    let leaf_bytes = F::from(BigUint::from_bytes_be(&leaf_hex));

    let mut proof = tree.create_proof(leaf_bytes);

    // Encode the Merkle Proof output with BigEndian 
    let mut merkle_siblings = Vec::with_capacity(1 * depth);
    let mut merkle_indices = Vec::with_capacity(1 * depth);
    
    //let siblings_bytes = proo
}

#[wasm_bindgen]
pub fn generate_merkle_proof(leaves: Vec<String>, leaf: String, depth: usize) {
    
}
