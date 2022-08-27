extern crate regex;

mod modules;

use modules::datatypes::Node;
use modules::settingshandler::parse_settings_arg;
use modules::parser::parse_source;

use std::collections::HashMap;
use std::fs::{self};

//This declarative language acts as a preprocessor for earwig
//settings from settings.EWS will be sent via cli args[2]

use std::env;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    //let node_arena = Arena::<Node>::default();
    //Open origin file for parsing
    let node_global = Node {
        value: String::from("SCOPE_GLOBAL"),
        render: false,
        tab_number: -1,
        scope: Box::new(HashMap::from([])),
        args:  Box::new(vec![])
    };

    let _settings_map = parse_settings_arg(cli_args[2].to_string());

    let file = fs::read_to_string(cli_args[1].to_string())
        .expect("Failed to open .ear file.");

    /*let mut origin_reader = BufReader::new(file);
    let mut exec_str = Vec::new();
    origin_reader.read_to_end(&mut exec_str).expect("Insert failed to read .ear file to string.");
    let s = match str::from_utf8(exec_str.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };*/
    parse_source(format!("{}\n$",file), node_global, cli_args[1].to_string().clone()).borrow_mut().interpret(false, cli_args[1].to_string().clone());
}