use crate::utils;
use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Clone, Parser, Deserialize)]
#[clap(name = "cfr-proto")]
#[clap(about = "Prototype of counterfactual regret minimization for trading")]
pub struct CliArgs {
    /// Print config
    #[clap(short = 'p', long = "print-config", default_value = "false")]
    pub print_config: bool,

    /// Config path
    #[clap(short = 'c', long = "config", default_value = "io/config.toml")]
    pub config_path: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub print_config: bool,
    pub dataset_path: String,
    pub start_iteration: u64,
    pub n_iterations: u64,
    pub iteration: IterationConfig,
}

impl Config {
    pub fn parse() -> anyhow::Result<Self> {
        let cli: CliArgs = CliArgs::parse();
        let config_path: String = utils::canonicalize_path(&cli.config_path)?;
        let config_text: String = std::fs::read_to_string(&config_path)?;
        let toml: Toml = toml::from_str(&config_text)?;

        let config = Config {
            print_config: cli.print_config,
            dataset_path: toml.dataset_path,
            start_iteration: toml.start_iteration,
            n_iterations: toml.n_iterations,
            iteration: toml.iteration,
        };
        Ok(config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Toml {
    pub dataset_path: String,
    pub start_iteration: u64,
    pub n_iterations: u64,
    pub iteration: IterationConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IterationConfig {
    pub n_plays: u64,
    pub concurrency: u64,
    pub output_dir: String,
    pub fee_per_contract_usd: f64,
    pub multiplier: f64,
    pub utility_penalty_bps: f64,
    pub max_play_duration_in_bars: u64,
}
