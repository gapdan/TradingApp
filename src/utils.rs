use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub command: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}
#[derive(Debug)]
pub struct Wallet {
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

#[derive(Debug)]
pub struct Transaction {
    pub sum: f64,
    pub client_id: u16,
}

pub enum OperationType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    ChargeBack,
    None,
}

impl Command {
    pub fn parse_opcode(&self) -> OperationType {
        match self.command.as_ref() {
            "deposit" => OperationType::Deposit,
            "withdrawal" => OperationType::Withdrawal,
            "dispute" => OperationType::Dispute,
            "resolve" => OperationType::Resolve,
            "chargeback" => OperationType::ChargeBack,
            _ => OperationType::None,
        }
    }
}
