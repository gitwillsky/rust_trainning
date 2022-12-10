use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::pb::{Block, BlockHash};

const PREFIX_ZERO: &[u8] = &[0, 0, 0];

pub fn pow(block: Block) -> Option<BlockHash> {
    let hasher = blake3_hash_base(&block.data);
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), *n);
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });

    nonce.map(|n| BlockHash {
        id: get_block_id(&block),
        hash: blake3_hash(hasher, n),
        nonce: n,
    })
}

fn get_block_id(block: &Block) -> Vec<u8> {
    blake3::hash(&block.data).as_bytes().to_vec()
}

fn blake3_hash(mut hasher: blake3::Hasher, nonce: u32) -> Vec<u8> {
    hasher.update(nonce.to_be_bytes().as_slice());
    hasher.finalize().as_bytes().to_vec()
}

fn blake3_hash_base(data: &[u8]) -> blake3::Hasher {
    let mut hash = blake3::Hasher::new();

    hash.update(data);
    hash
}
