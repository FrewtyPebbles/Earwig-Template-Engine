mod modules;

use modules::datatypes::Node;

use modules::parser::parse_source;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};

//This functional language acts as a preprocessor for earwig
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
        scope: HashMap::from([]),
        args:  vec![]
    };

    

    let file = File::open(cli_args[1].to_string())
        .expect("Failed to open .ear file.");

    let origin_reader = BufReader::new(file);
    parse_source(origin_reader, node_global).borrow_mut().interpret();
}