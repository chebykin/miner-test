extern crate ethcore;
extern crate ethereum_types;
extern crate ethkey;
extern crate ethcore_logger;
extern crate ethcore_io as io;
extern crate kvdb;
extern crate kvdb_memorydb;

#[macro_use]
extern crate log;
extern crate clap;

use ethcore::client::{Client, ClientConfig};
use ethcore::miner::{Miner, MinerService, TransactionImportResult};
use ethcore::spec::spec::Spec;
use ethcore::transaction::{Action, PendingTransaction, SignedTransaction, Transaction};
use std::sync::Arc;
use io::*;
use std::time;
use clap::App;
use clap::Arg;

fn transaction_with_chain_id(count: u64, chain_id: Option<u64>) -> Vec<SignedTransaction> {
    // 0x252Dae0A4b9d9b80F504F6418acd2d364C0c59cD
    let secret = "0000000000000000000000000000000000000000000000000000000000000011".parse().unwrap();
    let now = time::Instant::now();

    let mut txs: Vec<SignedTransaction> = Vec::with_capacity(count as usize);
    // TODO: take nonce from account
    let mut nonce = "0".parse().unwrap();

    for _ in 0..count {
        let t = Transaction{
            nonce: nonce,
            gas_price: 0.into(),
            gas: "21000".parse().unwrap(),
            action: Action::Create,
            value: "340000000000000000".parse().unwrap(),
            data: Vec::new(),
        };

        nonce = nonce + 1.into();
        txs.push(t.sign(&secret, chain_id))
    }

    println!("Generation of {:?} txs took {} seconds", count, now.elapsed().as_secs());
    return txs;
}

fn new_db() -> Arc<::kvdb::KeyValueDB> {
    Arc::new(::kvdb_memorydb::create(ethcore::db::NUM_COLUMNS.unwrap_or(0)))
}

fn main() {
    let matches = App::new("My Super Program")
        .version("1.9.6")
        .author("Nikita Chebykin <nikita@chebyk.in")
        .about("Parity miner test")
        .arg(Arg::with_name("count")
            .short("c")
            .long("count")
            .value_name("COUNT")
            .help("Sets # of txs to insert")
            .takes_value(true))
        .arg(Arg::with_name("logs")
            .short("l")
            .long("logs")
            .value_name("LOGS")
            .help("logs")
            .takes_value(true))
        .get_matches();

    let count = matches.value_of("count").unwrap().parse().unwrap();
    println!("Value for count: {}", count);

    ::ethcore_logger::init_log();
    let bytes: &[u8] = include_bytes!("../res/spec.json");
    let spec = Spec::load(&::std::env::temp_dir(), bytes).expect("invalid chain spec");

    let client_db = new_db();

    let miner = Miner::with_spec(&spec);
    let client = Client::new(ClientConfig::default(),
                             &spec,
                             client_db,
                             Arc::new(miner),
                             IoChannel::disconnected(),).unwrap();
    let unwrapped_client = Arc::try_unwrap(client).ok().expect("should unwrap");

    let txs = transaction_with_chain_id(count,Some(15054));

    let now = time::Instant::now();

    for tx in txs {
        let res = unwrapped_client.miner().import_own_transaction(
            &unwrapped_client, PendingTransaction::new(tx, None));

        assert_eq!(res.unwrap(), TransactionImportResult::Current);
    };

    println!("Adding transactions took {} seconds", now.elapsed().as_secs());

    assert_eq!(unwrapped_client.miner().pending_transactions().len(), count as usize);
}
