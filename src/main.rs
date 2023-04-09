use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, io};

use libyni::to_base_str;

const DEFAULT_OUTPUT_FORMAT: u32 = 2;

fn main() {
    let mut arg_iter = env::args().skip(1);
    let mut source_file = None;
    let mut output_format = DEFAULT_OUTPUT_FORMAT;

    while let Some(a) = arg_iter.next() {
        match a.as_str() {
            "-f" => {
                if arg_iter.len() == 0 {
                    println!("Wrong usage of '-f', ignoring...");
                }
                source_file = arg_iter.next();
            }
            "-o" => {
                if let Some(x) = arg_iter.next() {
                    if let Ok(parsed) = x.parse() {
                        if parsed >= 2 && parsed <= 36 {
                            output_format = parsed;
                        } else {
                            println!("Arguemnt of '-o', N should be 36 >= N >= 2, ignoring...");
                        }
                    } else {
                        println!("Wrong usage of '-o', ignoring...");
                    }
                } else {
                    println!("Wrong usage of '-o', ignoring...");
                }
            }
            _ => {}
        }
    }

    let mut buf = String::new();
    if source_file.is_some() {
        let file_path = PathBuf::from(source_file.unwrap());

        if let Ok(mut f) = File::open(&file_path) {
            f.read_to_string(&mut buf).unwrap();
        } else {
            println!("Cannot Open File {}", file_path.to_str().unwrap());
            return;
        }
    } else {
        let mut stdin = io::stdin().lock();

        println!("^d(EOF) for finish the input.");
        stdin.read_to_string(&mut buf).unwrap();
    }

    println!("\nUTF-8, {}진수", output_format);
    println!("{}", to_base_str(buf.as_str(), output_format));
}
