use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod parser;

use count::count_tokens;
use error::Error;

fn counts(files: Vec<PathBuf>) -> BTreeMap<parser::Language, u64> {
    files
        .into_iter()
        .map(|path| match parser::parse(&path) {
            Ok((tree, lang)) => Ok((lang, count_tokens(&tree))),
            Err(err) => Err(err),
        })
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .fold(BTreeMap::new(), |mut acc, (lang, count)| {
            *acc.entry(lang).or_insert(0) += count;
            acc
        })
}

fn run(cli: cli::Cli) -> Result<(), Error> {
    counts(cli.files)
        .iter()
        .for_each(|(lang, count)| println!("{:?}: {}", lang, count));
    Ok(())
}

fn main() {
    let cli = cli::Cli::from_args();

    match run(cli) {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
