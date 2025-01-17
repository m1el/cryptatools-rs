#![feature(array_windows)]
mod cli_structure;
use cli_structure::{Cli, Commands};

use std::collections::HashMap;
use clap::Parser;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::Statistical;

#[serde_as]
#[derive(Deserialize, Debug)]
struct Freq (
    #[serde_as(as = "HashMap<_, Vec<DisplayFromStr>>")] 
    HashMap<String, Vec<u8>>
);

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::GetCoincidenceIndex { cipher_text } => {
            println!("here is the coincidence index.")
            //let c = CoincidenceIndexGuesser::new(alphabet);
            //let coincidence_index: f64 = c.guess_coincidence_index(opcodes.as_bytes().to_vec());
        },
        Commands::FrequencyAnalysis { opcodes_cli_string, alphabet } => {
            let alphabet_object: HashMap<String, Vec<u8>> = match alphabet.clone().as_deref() {
                Some(alphabet_string_json) => {
                    let deserialize_object: Freq = serde_json::from_str(alphabet_string_json).unwrap();
                    deserialize_object.0
                },
                None => panic!("error"),
            };

            let opcodes = match opcodes_cli_string.as_deref() {
                Some(opcodes_string) => opcodes_string,
                None => panic!("cipher text opcodes not set."),
            };

            let mut opcodes_u8: Vec<u8> = vec![];
            for [a, b] in opcodes.split("").collect::<Vec<&str>>().array_windows().skip(1).step_by(2) {
                let mut str = String::from(a.clone());
                str.push_str(b.clone());
                let my_int = str.parse::<u8>().unwrap();
                opcodes_u8.push(my_int);
            }

            let stat = Statistical::new(alphabet_object);
            let result = stat.guess_statistical_distribution(opcodes_u8);

            let mut val = result.iter().collect::<Vec<_>>();
            val.sort_by_key(|value| value.0);

            for (k, v) in val {
                let mut str = String::from("");
                for opcode in k.iter() {
                    str.push_str(&String::from(opcode.to_string()));
                }
                println!("{}: {}", str, v);
            }
        },
    }
}