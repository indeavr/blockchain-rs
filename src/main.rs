use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        },
    ], difficulty);

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 360,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ],
        },
    ], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add block");
}

// fn main() {
//     let difficulty = 0x000fffffffffffffffffffffffffffff;
//
//     let mut genesis = Block::new(0, 1234, vec![0; 32], 0,
//                                  String::from("Genesis Block"), difficulty);
//     genesis.mine();
//     println!("Genesis Block: {:?}", &genesis);
//
//     let mut last_hash = genesis.hash.clone();
//
//     let mut blockchain = Blockchain {
//         blocks: vec![genesis]
//     };
//
//     println!("(Gen) Valid: {}", &blockchain.verify());
//
//     for i in 1..=10 {
//         let mut block = Block::new(i, now(), last_hash, 0,
//                                    String::from("Just a Block"), difficulty);
//         block.mine();
//         println!("Block{}: {:?}", i, &block);
//
//         last_hash = block.hash.clone();
//         blockchain.blocks.push(block);
//
//         println!("Valid: {}", &blockchain.verify());
//     }
//
//     // blockchain.blocks[3].index = 4;
//     // blockchain.blocks[3].hash[8] += 1;
//     // blockchain.blocks[3].payload = "Nope".to_owned();
//     blockchain.blocks[3].prev_block_hash[18] = 8;
//
//     println!("Valid: {}", &blockchain.verify());
//
//     // println!("Blockchain : {:?}", &blockchain);
// }
