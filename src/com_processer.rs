use crate::com_executor::{charge_back, deposit, dispute, resolve, withdrawal};
use crate::utils::{Command, OperationType, Transaction, Wallet};
use csv::{Position, StringRecordsIter};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

/**
 * Dispatch the command from the csv file and call
 * a specific function for each operation
*/
pub fn process_command(
    operation: &Command,
    transaction_pos: &HashMap<u32, Position>,
    wallets: &mut HashMap<u16, Wallet>,
    disputed_tx: &mut HashMap<u32, Transaction>,
    iter: &mut StringRecordsIter<File>,
) -> Result<(), Box<dyn Error>> {
    match operation.parse_opcode() {
        OperationType::Deposit => {
            deposit(&operation.client, &operation.amount.unwrap(), wallets);
            Ok(())
        }
        OperationType::Withdrawal => {
            withdrawal(&operation.client, &operation.amount.unwrap(), wallets);
            Ok(())
        }
        OperationType::Dispute => {
            dispute(
                &operation.client,
                &operation.tx,
                wallets,
                transaction_pos,
                disputed_tx,
                iter,
            );
            Ok(())
        }
        OperationType::Resolve => {
            resolve(&operation.client, &operation.tx, wallets, disputed_tx);
            Ok(())
        }
        OperationType::ChargeBack => {
            charge_back(&operation.client, &operation.tx, wallets, disputed_tx);
            Ok(())
        }
        OperationType::None => Err(From::from("[WARN] Invalid command parsed")),
    }
}

/**
 *  Return a specific command from the csv by seeking it
 *  into the file.
 *  The transaction_pos HashMap stores the position in file
 *  of each transaction.
 */
pub fn get_transaction(
    iter: &mut StringRecordsIter<File>,
    transaction_pos: &HashMap<u32, Position>,
    transaction: u32,
) -> Result<Command, Box<dyn Error>> {
    let crt_pos = iter.reader().position().clone();
    match transaction_pos.get(&transaction) {
        Some(pos) => {
            iter.reader_mut().seek(pos.clone())?;
        }
        None => {
            eprintln!("[WARN] Transaction {} inexistent", transaction);
            ()
        }
    }
    let mut line = Command {
        command: "".to_string(),
        client: 0,
        tx: 0,
        amount: Some(0.0),
    };
    match iter.next() {
        Some(result) => match result {
            Ok(command) => {
                line = command.deserialize(None)?;
            }
            Err(e) => {
                eprintln!("[ERROR] {}", e);
            }
        },
        None => {
            eprintln!("[ERROR] Nothing to parse in file");
        }
    }
    // Move the iterator back
    iter.reader_mut().seek(crt_pos)?;
    Ok(line)
}
