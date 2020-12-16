/// The actual Blockchain container
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Stores all the blocks which are accepted already within the blockchain
    pub blocks: Vec<Block>,

    /// Lookup from AccountID (will be a public key later) to Account.
    /// Effectively, this represents the WorldState
    pub accounts: HashMap<String, Account>,

    /// Will store transactions which should be added to the chain
    /// but aren't yet
    pending_transactions: Vec<Transaction>
}

/// Represents the current state of the blockchain after all Blocks are executed
/// A world state is technically not necessary since we always could build the information
/// by iterating through all the blocks.  Generally, this doesn't seem like a good option
/// However, we do not force the actual Blockchain to implement a WorldState but rather
/// behave like having one.  This trait therefore just defines an expected interface into our Blockchain.
/// (Actually it doesn't even care if the information is stored within a blockchain)
trait WorldState {
    /// Will bring us all registered user ids
    fn get_user_ids(&self) -> Vec<String>;

    /// Will return an account given it's id if it is available (mutable)
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    ///Will return an account given it's id if it is available
    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    /// Will add a new account
    fn create_account(&mut self, id: String, account_type: AccountType) -> Result<(), &'static str>;
}

/// Stores a request to the blockchain
#[derive(Clone, Debug)]
pub struct Transaction {

    /// Unique number (will be used for randomization later; preventes replay attacks)
    nonce: u128,

    /// Account ID
    from: String,

    /// Stores the time the transaction was created
    created_at: SystemTime,

    /// the type of the transaction and its additional information
    pub(crate) record: TransactionData,

    /// Signature of the hash of the whole message
    signature: Option<String>,
}