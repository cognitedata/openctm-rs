use clap::clap_app;
use openctm;
use serde_json;
use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(("openctm-dump") =>
        (@arg file: +takes_value)
        (@arg yaml: -y --yaml)
        (@arg compact: -c --compact)
    )
    .get_matches();

    let file = matches.value_of("file");
    let use_yaml = matches.is_present("yaml");
    let use_compact = matches.is_present("compact");

    let reader: Box<dyn Read> = if let Some(file) = file {
        Box::new(File::open(file)?)
    } else {
        Box::new(stdin())
    };

    let result = openctm::parse(std::io::BufReader::new(reader))?;

    if use_compact {
        serde_json::to_writer(stdout(), &result)?;
    } else if use_yaml {
        serde_yaml::to_writer(stdout(), &result)?;
    } else {
        serde_json::to_writer_pretty(stdout(), &result)?;
    }

    Ok(())
}
