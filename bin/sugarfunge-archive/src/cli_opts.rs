use std::{borrow::Cow, fs, path::PathBuf};

use anyhow::Result;
use argh::FromArgs;

use substrate_archive::ArchiveConfig;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_chain_spec::GenericChainSpec<sugarfunge_runtime::GenesisConfig>;

/// Node Template Archiver for development use.
#[derive(Clone, FromArgs)]
pub struct CliOpts {
	/// sets a custom config file
	#[argh(option, short = 'c', long = "config")]
	pub config: Option<PathBuf>,
	/// sets spec for chain from a JSON file. Runs in `dev` mode by default.
	#[argh(option, default = "default_chain_spec()", short = 's', long = "spec", from_str_fn(parse_chain_spec))]
	pub chain_spec: ChainSpec,
}

fn parse_chain_spec(path: &str) -> Result<ChainSpec, String> {
	ChainSpec::from_json_file(PathBuf::from(path))
}

fn default_chain_spec() -> ChainSpec {
	let file = include_bytes!("./sugarfunge-spec.json");
	let file: Cow<'static, [u8]> = Cow::Borrowed(file);
	ChainSpec::from_json_bytes(file).expect("Default ChainSpec `dev` could not be loaded")
}

impl CliOpts {
	pub fn init() -> Self {
		argh::from_env()
	}

	pub fn parse(&self) -> Result<Option<ArchiveConfig>> {
		if let Some(config) = &self.config {
			let toml_str = fs::read_to_string(config.as_path())?;
			let config = toml::from_str::<ArchiveConfig>(toml_str.as_str())?;
			Ok(Some(config))
		} else {
			Ok(None)
		}
	}
}
