mod cli_opts;

use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

use sugarfunge_runtime::{opaque::Block, RuntimeApi};

use substrate_archive::{Archive, ArchiveBuilder, SecondaryRocksDb};

fn main() -> anyhow::Result<()> {
	let cli = cli_opts::CliOpts::init();
	let config = cli.parse()?;

	let mut archive = ArchiveBuilder::<Block, RuntimeApi, SecondaryRocksDb>::with_config(config)
		.chain_spec(Box::new(cli.chain_spec))
		.build()?;
	archive.drive()?;

	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl-C handler");
	while running.load(Ordering::SeqCst) {}
	archive.shutdown()?;
	Ok(())
}
