use itertools::Itertools;
use clap::Parser;
use std::collections::HashMap;
use log::{info, warn};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
mod io;
use std::io::{stdin};

/// Count the frequencies of words in text file(s) or stdin
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the .txt data files. Not required when using stdin
    data_path: Vec<String>,

    /// Lower limit for the number of occurences of a word to be included
    #[clap(short, long, default_value_t = 5)]
    limit: u32,

    /// Output frequencies instead of total counts
    #[clap(short, long, action)]
    freqs: bool,

    /// Remove punctuation ('(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!')
    #[clap(short, long, action)]
    clean: bool,

    /// Remove numbers
    #[clap(long, action)]
    clean_numbers: bool,

    /// Convert all words to lowercase
    #[clap(long, action)]
    lower: bool,

}

fn main() {
    let punctuation = ['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!'];
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let args = Args::parse();
    let mut words: Vec<String> = vec![];

    let no_of_files = args.data_path.len();
    let filebar = ProgressBar::new(no_of_files as u64);
    for data_path in args.data_path {
        let contents = io::read_lines(data_path);
        if let Ok(lines) = contents {
            for line in lines {
                if let Ok(clean_line) = line {
                    let tokens = clean_line.split_whitespace();
                    for token in tokens {
                        let mut clean_token = token.to_string();
                        if args.clean {
                            clean_token = token.replace(&punctuation[..], "");
                        }
                        if args.clean_numbers {
                            clean_token = clean_token.replace(&numbers[..], "");
                        }
                        if args.lower {
                            clean_token = clean_token.to_lowercase();
                        }
                        words.push(clean_token);
                    }
                }
            }
        }
        filebar.inc(1);
    }
    filebar.finish();

    let stdin_lines = std::io::stdin().lines();

    let stdinbar = ProgressBar::no_length();
    for line in stdin_lines {
        //eprintln!("got a line: {}", line.unwrap());
        if let Ok(clean_line) = line {

            let tokens = clean_line.split_whitespace();
            for token in tokens {
                let mut clean_token = token.to_string();
                if args.clean {
                    clean_token = token.replace(&punctuation[..], "");
                }
                if args.clean_numbers {
                    clean_token = clean_token.replace(&numbers[..], "");
                }
                if args.lower {
                    clean_token = clean_token.to_lowercase();
                }
                words.push(clean_token);
            }
        }
        filebar.inc(1);
    }

    let N = words.len();
    if N == 0 {
        warn!("Warning! {}!", "the specified file(s) contain no text");
        eprintln!("warning: the specified file(s) contain no text");
    }

    let limit = args.limit;
    let counts: HashMap<&String, usize> = words.iter().progress().counts().into_iter().progress().filter(|(k,v)| v >= &&(limit as usize)).collect();
    if !args.freqs {
        let serialization_result = serde_json::to_string_pretty(&counts);

        match serialization_result {
            Ok(v) => println!("{}", v),
            Err(e) => eprintln!("error parsing header: {e:?}"),
        }
    } else {
        let freqs: HashMap<&String, f32> = counts.into_iter().map(|(k,v)| (k, v as f32 / (N as f32))).collect();
        let serialization_result = serde_json::to_string_pretty(&freqs);
        match serialization_result {
            Ok(v) => println!("{}", v),
            Err(e) => eprintln!("error parsing header: {e:?}"),
        }

    }
}
