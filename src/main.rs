use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis = Block::new(0, 1234, vec![0; 32], 0,
                                 String::from("Genesis Block"), difficulty);
    genesis.mine();
    println!("Genesis Block: {:?}", &genesis);

    let mut last_hash = genesis.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![genesis]
    };

    println!("(Gen) Valid: {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, 0,
                                   String::from("Just a Block"), difficulty);
        block.mine();
        println!("Block{}: {:?}", i, &block);

        last_hash = block.hash.clone();
        blockchain.blocks.push(block);

        println!("Valid: {}", &blockchain.verify());
    }

    // blockchain.blocks[3].index = 4;
    // blockchain.blocks[3].hash[8] += 1;
    // blockchain.blocks[3].payload = "Nope".to_owned();
    blockchain.blocks[3].prev_block_hash[18] = 8;

    println!("Valid: {}", &blockchain.verify());

    // println!("Blockchain : {:?}", &blockchain);
}
