extern crate ethcore;
extern crate ethereum_types;

use ethcore::miner::{Miner, MinerService};
use std::env;
use std::sync::Arc;
use ethcore::miner::MinerOptions;
use std::time::Duration;
use ethcore::miner::GasLimit;
use ethcore::miner::PrioritizationStrategy;
use ethcore::miner::PendingSet;
use ethcore::miner::Banning;
use ethcore::miner::GasPricer;
use ethcore::spec::spec::Spec;

fn miner() -> Miner {
    Arc::try_unwrap(Miner::new(
        MinerOptions {
            new_work_notify: Vec::new(),
            force_sealing: false,
            reseal_on_external_tx: false,
            reseal_on_own_tx: true,
            reseal_on_uncle: false,
            reseal_min_period: Duration::from_secs(5),
            reseal_max_period: Duration::from_secs(120),
            tx_gas_limit: "4700000".parse().unwrap(),
            tx_queue_size: 1024,
            tx_queue_memory_limit: None,
            tx_queue_gas_limit: GasLimit::None,
            tx_queue_strategy: PrioritizationStrategy::GasFactorAndGasPrice,
            pending_set: PendingSet::AlwaysSealing,
            work_queue_size: 5,
            enable_resubmission: true,
            tx_queue_banning: Banning::Disabled,
            refuse_service_transactions: false,
            infinite_pending_block: false,
        },
        GasPricer::new_fixed(0u64.into()),
        &Spec::new_instant(),
        None, // accounts provider
    )).ok().expect("Miner was just created.")
}

fn main() {
    let _miner = miner();
    println!("got miner")
}
