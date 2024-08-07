#![allow(missing_docs, rustdoc::missing_crate_level_docs)]

use clap::Parser;
use reth::cli::Cli;
use reth_node_optimism::{args::RollupArgs, OptimismNode};

// We use jemalloc for performance reasons
#[cfg(all(feature = "jemalloc", unix))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(not(feature = "optimism"))]
compile_error!("Cannot build the `op-reth` binary with the `optimism` feature flag disabled. Did you mean to build `reth`?");

#[cfg(feature = "optimism")]
fn main() {
    reth_cli_util::sigsegv_handler::install();

    // Enable backtraces unless a RUST_BACKTRACE value has already been explicitly provided.
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    if let Err(err) = Cli::<RollupArgs>::parse().run(|builder, rollup_args| async move {
        let handle = builder
            .node(OptimismNode::new(rollup_args.clone()))
            .update_ext_args(rollup_args.sequencer_http)
            .launch()
            .await?;

        handle.node_exit_future.await
    }) {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
