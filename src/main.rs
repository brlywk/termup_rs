use crate::{cli::Cli, config::Config};
use anyhow::Result;
use clap::Parser;

mod cli;
mod config;

fn main() -> Result<()> {
    // let lines = pad_align_lines(
    //     &[
    //         ("This is the first line:", "Success"),
    //         ("Second line:", "Fail"),
    //         ("Third line incoming:", "Neutral"),
    //     ],
    //     10,
    // );
    //
    // println!("{}", lines);
    let cli = Cli::parse();
    let config_content = std::fs::read_to_string(&cli.config_file)?;
    let config: Config = toml::from_str(&config_content)?;

    println!("{}", config.info.name);

    Ok(())
}

fn pad_align_lines(lines: &[(&str, &str)], padding: usize) -> String {
    let left_max = lines.iter().map(|(lhs, _)| lhs.len()).max().unwrap_or(0);

    lines
        .iter()
        .map(|(lhs, rhs)| {
            let total_width = left_max + padding;
            format!("{:<width$}{}", lhs, rhs, width = total_width)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
