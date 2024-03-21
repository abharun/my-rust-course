// pub struct TxPayload

pub struct Address(String);

pub struct Signature(String);

pub struct TxPayload {
    pub addr: Address,
    pub amount: i32,
}

pub struct Transaction {
    pub timestamp: String,
    pub nonce: u64,
    pub payload: TxPayload,
    pub signer: Address,
    pub signature: Signature,
}
