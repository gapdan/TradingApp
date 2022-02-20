mod com_executor;
mod com_processer;
mod utils;

use crate::com_processer::process_command;
use crate::utils::{Transaction, Wallet};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::process;

fn print_results(wallets: &HashMap<u16, Wallet>) -> Result<(), Box<dyn Error>> {
    let mut wrtr = csv::Writer::from_writer(io::stdout());
    wrtr.write_record(&["client", "available", "held", "total", "locked"])?;
    for (client_id, wallet) in &*wallets {
        let c_id = format!("{}", client_id);
        let avail = format!("{:.4}", wallet.available);
        let held = format!("{:.4}", wallet.held);
        let total = format!("{:.4}", wallet.total);
        let locked = format!("{}", wallet.locked);
        wrtr.write_record(&[c_id, avail, held, total, locked])?
    }
    wrtr.flush()?;
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut transaction_pos = HashMap::new();
    let mut wallets: HashMap<u16, Wallet> = HashMap::new();
    let mut disputed_tx: HashMap<u32, Transaction> = HashMap::new();

    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b',')
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_path(file_path)?;
    // Read the headers to start with the iterator from the first line
    let _headers = rdr.headers()?;
    let mut iter = rdr.records();

    // Go line by line through the file and process commands
    loop {
        let pos = iter.reader().position().clone();
        match iter.next() {
            Some(result) => match result {
                Ok(command) => match command.deserialize(None) {
                    Ok(record) => {
                        process_command(
                            &record,
                            &transaction_pos,
                            &mut wallets,
                            &mut disputed_tx,
                            &mut iter,
                        )?;
                        transaction_pos.insert(record.tx, pos);
                    }
                    Err(e) => {
                        eprintln!("[ERROR] Couldn't deserialize command: {:?}", e);
                    }
                },
                Err(e) => {
                    eprintln!("[ERROR] {}", e);
                    break;
                }
            },
            None => {
                break;
            }
        }
    }

    print_results(&wallets)?;
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
