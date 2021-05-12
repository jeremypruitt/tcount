use crate::output::Format;
use crate::query::Query;
use regex::Regex;
use std::format;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tc",
    about = "Count your code by tokens, token kinds, and patterns in the syntax tree."
)]
pub struct Cli {
    #[structopt(
        long,
        help = "Logging level. 0 to not print errors. 1 to print IO and filesystem errors. 2 to print parsing errors. 3 to print everything else.",
        default_value = "0"
    )]
    pub verbose: u8,

    #[structopt(
        short,
        long,
        help = "kinds of node in the syntax tree to count. See node-types.json in the parser's repo."
    )]
    pub kind: Vec<String>,

    #[structopt(
        short = "p",
        long,
        help = "Patterns of node kinds to count in the syntax tree (e.g. \".*comment.*\" to match nodes of type \"line_comment\", \"block_comment\", and \"comment\"). Supports Rust regular expressions"
    )]
    pub kind_pattern: Vec<Regex>,

    #[structopt(long, help = "TODO")]
    pub query: Vec<Query>,

    #[structopt(
        long,
        default_value = "tokens",
        help = "One of group|numfiles|tokens. \"group\" will sort based on --group-by value"
    )]
    pub sort_by: SortBy,

    #[structopt(
        long,
        default_value = "language",
        help = "One of language|file|arg. \"arg\" will group by the `paths` arguments provided"
    )]
    pub group_by: GroupBy,

    #[structopt(long, default_value = "table", help = "One of table|csv")]
    pub format: Format,

    #[structopt(long, help = "Don't respect gitignore and .git/info/exclude files")]
    pub no_git: bool,

    #[structopt(long, help = "Don't respect .ignore files")]
    pub no_dot_ignore: bool,

    #[structopt(long, help = "Don't respect ignore files from parent directories")]
    pub no_parent_ignore: bool,

    #[structopt(long, help = "Count hidden files")]
    pub count_hidden: bool,

    #[structopt(
        long,
        help = "Whitelist of languages to parse. This overrides --blacklist and must be an exact match"
    )]
    pub whitelist: Vec<String>,

    #[structopt(
        long,
        help = "Blacklist of languages not to parse. This is overriden by --whitelist and must be an exact match"
    )]
    pub blacklist: Vec<String>,

    #[structopt(long, help = "Show a list of supported languages for parsing")]
    pub list_languages: bool,

    #[structopt(long, help = "Show column totals")]
    pub show_totals: bool,

    #[structopt(
        default_value = ".",
        help = "Files and directories to parse and count."
    )]
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SortBy {
    Group,
    NumFiles,
    Tokens,
}

impl FromStr for SortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "group" => Ok(SortBy::Group),
            "numfiles" => Ok(SortBy::NumFiles),
            "tokens" => Ok(SortBy::Tokens),
            _ => Err(format!(
                "\"{}\" is not a supported argument to --sort-by. Use one of group|numfiles|tokens",
                s
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GroupBy {
    Language,
    File,
    Arg,
}

impl FromStr for GroupBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "language" => Ok(GroupBy::Language),
            "file" => Ok(GroupBy::File),
            "arg" => Ok(GroupBy::Arg),
            _ => Err(format!(
                "\"{}\" is not a supported argument to --group-by. Use one of language|file",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_by_from_str() {
        assert_eq!(GroupBy::Language, GroupBy::from_str("language").unwrap());
        assert_eq!(GroupBy::File, GroupBy::from_str("file").unwrap());
        assert_eq!(GroupBy::Arg, GroupBy::from_str("arg").unwrap());
    }

    #[test]
    fn sort_by_from_str() {
        assert_eq!(SortBy::Group, SortBy::from_str("group").unwrap());
        assert_eq!(SortBy::NumFiles, SortBy::from_str("numfiles").unwrap());
        assert_eq!(SortBy::Tokens, SortBy::from_str("tokens").unwrap());
    }
}
