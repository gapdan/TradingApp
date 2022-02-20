use crate::com_processer::get_transaction;
use crate::utils::{Transaction, Wallet};
use csv::{Position, StringRecordsIter};
use std::collections::HashMap;
use std::fs::File;

pub fn deposit(client_id: &u16, amount: &f64, wallets: &mut HashMap<u16, Wallet>) {
    match wallets.get_mut(client_id) {
        Some(wallet) => {
            if !wallet.locked {
                if *amount > 0.0 {
                    wallet.available += *amount;
                    wallet.total += *amount;
                } else {
                    eprintln!("[ERROR] Try to deposit a positive sum");
                }
            } else {
                eprintln!("[WARN] Wallet locked, the account is frozen");
            }
        }
        None => {
            eprintln!("[INFO] New client found");
            let wall = Wallet {
                locked: false,
                available: *amount,
                held: 0.0,
                total: *amount,
            };
            wallets.insert(*client_id, wall);
        }
    }
}

pub fn withdrawal(client_id: &u16, amount: &f64, wallets: &mut HashMap<u16, Wallet>) {
    match wallets.get_mut(client_id) {
        Some(wallet) => {
            if !wallet.locked && wallet.available >= *amount {
                if *amount > 0.0 {
                    wallet.available -= *amount;
                    wallet.total -= *amount;
                } else {
                    eprintln!("[ERROR] Try to deposit a positive sum");
                }
            } else {
                eprintln!("[WARN] Not enough founds or wallet locked");
            }
        }
        None => {
            eprintln!("[WARN] Invalid client id, do nothing");
        }
    }
}

pub fn dispute(
    client_id: &u16,
    transaction_id: &u32,
    wallets: &mut HashMap<u16, Wallet>,
    transaction_pos: &HashMap<u32, Position>,
    disputed_tx: &mut HashMap<u32, Transaction>,
    iter: &mut StringRecordsIter<File>,
) {
    match get_transaction(iter, transaction_pos, *transaction_id) {
        Ok(comm) => {
            if comm.command == "deposit" && comm.client == *client_id {
                match wallets.get_mut(&comm.client) {
                    Some(wallet) => {
                        let amount = comm.amount.unwrap();
                        if !wallet.locked && wallet.available >= amount {
                            if amount > 0.0 {
                                wallet.available -= amount;
                                wallet.held += amount;
                                let transaction = Transaction {
                                    sum: amount,
                                    client_id: comm.client,
                                };
                                disputed_tx.insert(*transaction_id, transaction);
                            } else {
                                eprintln!("[ERROR] The transaction disputed has invalid data");
                            }
                        } else {
                            eprintln!("[WARN] Not enough founds");
                        }
                    }
                    None => {
                        eprintln!("[WARN] Invalid client id, do nothing");
                    }
                }
            } else {
                eprintln!("[WARN] Invalid command to dispute");
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}

pub fn resolve(
    client_id: &u16,
    transaction_id: &u32,
    wallets: &mut HashMap<u16, Wallet>,
    disputed_tx: &mut HashMap<u32, Transaction>,
) {
    if !disputed_tx.contains_key(transaction_id) {
        eprintln!(
            "[WARN] Dispute not started for transaction {:?}",
            transaction_id
        );
        return;
    }
    let transaction = disputed_tx.get(transaction_id).unwrap();
    if transaction.client_id != *client_id {
        eprintln!("[ERROR] Client tries to resolve another's client transaction");
    }
    match wallets.get_mut(&transaction.client_id) {
        Some(wallet) => {
            let amount = transaction.sum;
            if !wallet.locked && wallet.held >= amount {
                wallet.available += amount;
                wallet.held -= amount;
                // Remove the dispute after it's done
                disputed_tx.remove(transaction_id);
            } else {
                eprintln!("[WARN] Not enough founds or wallet locked");
            }
        }
        None => {
            eprintln!("[WARN] Invalid client id, do nothing");
        }
    }
}

pub fn charge_back(
    client_id: &u16,
    transaction_id: &u32,
    wallets: &mut HashMap<u16, Wallet>,
    disputed_tx: &mut HashMap<u32, Transaction>,
) {
    if !disputed_tx.contains_key(transaction_id) {
        eprintln!(
            "[WARN] Dispute not started for transaction {:?}",
            transaction_id
        );
        return;
    }
    let transaction = disputed_tx.get(transaction_id).unwrap();
    if transaction.client_id != *client_id {
        eprintln!("[ERROR] Client tries to charge back another's client transaction");
    }
    match wallets.get_mut(&transaction.client_id) {
        Some(wallet) => {
            let amount = transaction.sum;
            if !wallet.locked && wallet.total >= amount {
                wallet.held -= amount;
                wallet.total -= amount;
                wallet.locked = true;
                // Remove the dispute after it's done
                disputed_tx.remove(transaction_id);
            } else {
                eprintln!("[WARN] Not enough founds or wallet locked");
            }
        }
        None => {
            eprintln!("[WARN] Invalid client id, do nothing");
        }
    }
}
