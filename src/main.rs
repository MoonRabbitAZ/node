

//! moonrabbit CLI

#![warn(missing_docs)]

use color_eyre::eyre;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	cli::run()?;
	Ok(())
}
