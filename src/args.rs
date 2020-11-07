use std::fs;
use std::io::{self, Read};

use clap::{Arg, App};

use pete_core::common::variable::{Variable, VariableStore};

use crate::error::Error;

pub struct Args {
    pub debug_blocks: bool,
    pub params: VariableStore,
    pub template: String,
}

impl Args {
    pub fn create() -> Args {
        Args {
            debug_blocks: false,
            params: VariableStore::new(),
            template: String::new(),
        }
    }

    pub fn create_from_cli() -> Result<Args, Error> {
        let mut args = Args::create();

        let matches = App::new("Pete")
            .about("Parametrized Extendable Template Engine")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Input template file path. In not specified then template will be parsed from standard input")
                .takes_value(true))
            .arg(Arg::with_name("param")
                .short("p")
                .long("param")
                .multiple(true)
                .takes_value(true)
                .help("A parameter with KEY=VALUE format"))
            .arg(Arg::with_name("debug-blocks")
                .short("b")
                .long("debug-blocks")
                .help("If TRUE then prints the block structure instead of rendering blocks"))
            .get_matches();
        let file_option = matches.value_of("file");
    
        match file_option {
            Some(_) => { args.template = fs::read_to_string(file_option.unwrap())?; },
            None => {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                handle.read_to_string(&mut args.template)?;
            }
        };
    
        args.debug_blocks = matches.is_present("debug-blocks");
    
        let params = matches.values_of("param");
    
        let params: Vec<&str> = match params {
            Some(p) => p.collect(),
            None => Vec::new(),
        };
        for param in params {
            let pos = param.find("=");
            if pos.is_none() {
                return Err(Error::create(format!("The parameter '{}' is not a KEY=VALUE pair", param)));
            }
            let pos = pos.unwrap();
            let mut parameter = Variable::new();
            parameter.set_string_value(param[pos+1..param.len()].to_string());
            args.params.insert(param[0..pos].to_string(), parameter);
        }
    
        Ok(args)
    }
}