use super::*;
use std::collections::HashSet;
use std::hash::Hash;
use crate::block::check_difficulty;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
        }
    }

    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("<<< I:{} >>>", i);
            if block.index != i as u32 {
                println!("index missmatch");
                return false;
            } else if !check_difficulty(&block.hash(), block.difficulty) {
                println!("diff not enought");
                return false;
            } else if i == 0 {
                // Genesis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("genesis prev block hash invalid: {:?} ... {:?}",&block.prev_block_hash, vec![0; 32]);
                    return false;
                }
            } else {
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("time did not increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("hash mismatch");
                    return false;
                }
            }
        }
        true
    }
}
