use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use regex::Regex;
use structopt::StructOpt;

fn main() {
    let opt = Opts::from_args();

    let mut output = HashSet::new();

    let r = Regex::new(r"\d{17,18}").unwrap();

    let input = {
        match File::open(opt.input) {
            Ok(mut f) => {
                let len = {
                    match f.metadata() {
                        Ok(m) => m.len(),
                        Err(err) => {
                            eprintln!("Error reading metadata:");
                            eprintln!("{}", err);
                            return;
                        }
                    }
                };
                let mut buf = Vec::with_capacity(len as usize);
                if let Err(err) = f.read_to_end(&mut buf) {
                    eprintln!("Error reading from file:");
                    eprintln!("{}", err);
                    return;
                }
                buf
            }
            Err(err) => {
                eprintln!("Error opening file:");
                eprintln!("{}", err);
                return;
            }
        }
    };

    let input = String::from_utf8_lossy(input.as_ref());

    for m in r.find_iter(input.as_ref()) {
        output.insert(u64::from_str(m.as_str()).unwrap());
    }

    match opt.output {
        Some(path) => {
            let mut file = match File::create(path) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Error opening output file:");
                    eprintln!("{}", err);
                    return;
                }
            };

            let vals = output
                .iter()
                .map(|v| format!("{}", *v))
                .collect::<Vec<String>>()
                .join("\n");

            if let Err(err) = file.write(vals.as_bytes()) {
                eprintln!("Error writing to output file:");
                eprintln!("{}", err);
            }
        }
        None => {
            let out = std::io::stdout();
            let mut out = out.lock();

            let csv = output
                .iter()
                .map(|v| format!("{}", *v))
                .collect::<Vec<String>>()
                .join(",");

            if let Err(err) = out.write(csv.as_bytes()) {
                eprintln!("Error writing to stdout:");
                eprintln!("{}", err);
                return;
            }
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "id_unique", about = "Get all unique IDs from a body of text")]
struct Opts {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}
