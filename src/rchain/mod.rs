// The actual Blockchain container
#[derive(Debug, Clone)]
pub struct Blockchain {
    // Stores all the blocks which are accepted already within the blockchain
    pub blocks: Vec<Block>,

    // Lookup from AccountID (will be a public key later) to Account.
    // Effectively, this represents the WorldState
    pub accounts: HashMap<String, Account>,

    // Will store transactions which should be added to the chain
    // but aren't yet
    pending_transactions: Vec<Transaction>
}