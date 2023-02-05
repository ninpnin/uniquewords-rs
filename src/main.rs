use itertools::Itertools;
use clap::Parser;
use std::collections::HashMap;

mod io;

//use crate::io;

/// Count the frequencies of words in text file(s)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the .txt data files
    #[clap(short, long)]
    data_path: Vec<String>,

    /// Lower limit for the number of occurences of a word to be included
    #[clap(short, long, default_value_t = 5)]
    limit: u32,
}

fn main() {

    let args = Args::parse();
    let mut words: Vec<String> = vec![];

    for data_path in args.data_path {
        let contents = io::read_lines(data_path);
        if let Ok(lines) = contents {
            for line in lines {
                if let Ok(clean_line) = line {
                    
                    let tokens = clean_line.split_whitespace();
                    for token in tokens {
                        words.push(token.to_string());
                    }
                }
            }
        }
    }

    let limit = args.limit;
    let freqs: HashMap<&String, usize> = words.iter().counts().into_iter().filter(|(k,v)| v >= &&(limit as usize)).collect();
    let serialization_result = serde_json::to_string_pretty(&freqs);

    match serialization_result {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("error parsing header: {e:?}"),
    }
}
