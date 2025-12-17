use alloy_primitives::{address, U256};
use revm::{
    primitives::{AccountInfo, Bytecode, TxKind},
    Evm, InMemoryDB,
};

fn main() {
    println!("=== REVM Simple Test ===\n");

    // Test 1: Simple value transfer
    test_simple_transfer();

    // Test 2: Contract execution (simple ADD operation)
    test_simple_contract();
}

fn test_simple_transfer() {
    println!("--- Test 1: Simple ETH Transfer ---");

    let sender = address!("1000000000000000000000000000000000000001");
    let receiver = address!("2000000000000000000000000000000000000002");

    // Create database and fund sender
    let mut db = InMemoryDB::default();
    db.insert_account_info(
        sender,
        AccountInfo {
            balance: U256::from(1_000_000_000_000_000_000u128), // 1 ETH
            nonce: 0,
            ..Default::default()
        },
    );

    // Build and run EVM
    let mut evm = Evm::builder()
        .with_db(db)
        .modify_tx_env(|tx| {
            tx.nonce = None; // Disable nonce check
        })
        .modify_tx_env(|tx| {
            tx.caller = sender;
            tx.transact_to = TxKind::Call(receiver);
            tx.value = U256::from(100_000_000_000_000_000u128); // 0.1 ETH
            tx.gas_limit = 21000;
            tx.gas_price = U256::from(1);
        })
        .build();

    let result = evm.transact_commit();

    match result {
        Ok(result) => {
            println!("  Transfer successful!");
            println!("  Success: {}", result.is_success());

            // Check receiver balance
            let receiver_account = evm.db().accounts.get(&receiver);
            if let Some(acc) = receiver_account {
                println!("  Receiver balance: {} wei", acc.info.balance);
            }
        }
        Err(e) => {
            println!("  Transfer failed: {:?}", e);
        }
    }
    println!();
}

fn test_simple_contract() {
    println!("--- Test 2: Simple Contract Execution ---");

    let caller = address!("1000000000000000000000000000000000000001");
    let contract_addr = address!("3000000000000000000000000000000000000003");

    // Simple bytecode: PUSH1 0x02, PUSH1 0x03, ADD, PUSH1 0x00, MSTORE, PUSH1 0x20, PUSH1 0x00, RETURN
    // This pushes 2 and 3, adds them (result: 5), stores in memory, and returns
    let bytecode = vec![
        0x60, 0x02, // PUSH1 0x02
        0x60, 0x03, // PUSH1 0x03
        0x01,       // ADD (2 + 3 = 5)
        0x60, 0x00, // PUSH1 0x00
        0x52,       // MSTORE (store result at memory position 0)
        0x60, 0x20, // PUSH1 0x20 (32 bytes)
        0x60, 0x00, // PUSH1 0x00 (offset 0)
        0xf3,       // RETURN
    ];

    // Create database
    let mut db = InMemoryDB::default();

    // Fund caller
    db.insert_account_info(
        caller,
        AccountInfo {
            balance: U256::from(1_000_000_000_000_000_000u128),
            nonce: 0,
            ..Default::default()
        },
    );

    // Deploy contract
    db.insert_account_info(
        contract_addr,
        AccountInfo {
            code: Some(Bytecode::new_raw(bytecode.into())),
            ..Default::default()
        },
    );

    // Build and run EVM
    let mut evm = Evm::builder()
        .with_db(db)
        .modify_tx_env(|tx| {
            tx.nonce = None; // Disable nonce check
        })
        .modify_tx_env(|tx| {
            tx.caller = caller;
            tx.transact_to = TxKind::Call(contract_addr);
            tx.gas_limit = 100_000;
            tx.gas_price = U256::from(1);
        })
        .build();

    let result = evm.transact_commit();

    match result {
        Ok(result) => {
            println!("  Contract call successful!");
            println!("  Success: {}", result.is_success());
            if let Some(output) = result.output() {
                let value = U256::from_be_slice(output);
                println!("  Return value: {} (expected: 5)", value);
            }
        }
        Err(e) => {
            println!("  Contract call failed: {:?}", e);
        }
    }
}
