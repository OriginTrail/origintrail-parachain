mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod inherents;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}