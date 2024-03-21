use super::transaction::Transaction;

pub struct MemPool {
    pub txpool: Vec<Transaction>,
}
