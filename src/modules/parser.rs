use std::fs::File;
use std::io::{ BufRead, BufReader};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::modules::parserutilities::{parameter_determine};
use crate::modules::datatypes::{Parameter, Node};

pub fn parse_source(src:BufReader<File>, node_global:Node) -> Rc<RefCell<Node>>{
	let mut syntax_buffer:String = String::from("");
    let mut nodestack = Vec::new();
    nodestack.push(Rc::new(RefCell::new(node_global)));
    for line in src.lines() {
        let line_val =  line.expect("Failed to read line.");
        if !line_val.chars().all(char::is_whitespace)
        {
        let mut node_new = Node {
            value: String::from(""),
            render: false,
            tab_number: 0,
            scope: HashMap::from([]),
            args: vec![]
        };
        let mut is_args:bool = false;
        let mut is_commenting:bool = false;
		let mut is_quoting:bool = false;
		let mut last_char:char = 'z';
		let mut force_string:bool = false;
        for origin_char in line_val.chars()
        {
            if !is_commenting
            {
				if origin_char == '"' && last_char != '\\'
				{
					is_quoting = !is_quoting;
				}
				if !is_quoting && origin_char != '"'
				{
					match origin_char {
						':' => {
							if !is_args {
								node_new.value = syntax_buffer.clone();
								is_args = true;
								syntax_buffer.clear();
							}
							else{
								syntax_buffer.push(origin_char);
							}
						},
						'?' => {
							node_new.render = true;
						},
						'#' => {
							is_commenting = true;
						},
						' ' => {
							if is_args && syntax_buffer != "" {
								//run tests on syntax_buffer to decern what type it is for var_type
								//these tests should be in a function
								//the fallback should be "str"
								let parameter_new = Parameter{
									value: syntax_buffer.clone(),
									var_type: String::from(parameter_determine(syntax_buffer.clone(), force_string))
								};
								node_new.args.push(parameter_new);
							}
							syntax_buffer.clear();
						},
						'\t' => {
							node_new.tab_number += 1;
						},
						_ => {
							force_string = false;
							syntax_buffer.push(origin_char);
							if !is_args {
								let last_ind = nodestack.len() - 1;
								if node_new.tab_number <= nodestack[last_ind].borrow().tab_number {
									nodestack.pop();
								}
							}
						}
					}
				}
				else if is_quoting {
					force_string = true;
					if !(last_char != '\\' && origin_char == '"')
					{
						syntax_buffer.push(origin_char.clone());
					}
				}
            }
			last_char = origin_char.clone();
        }
        if !is_commenting{
            //End of line (this is where \n will be handled.)
            if is_args && syntax_buffer != "" {
                //run tests on syntax_buffer to decern what type it is for var_type
                //these tests should be in a function
                //the fallback should be string
                let parameter_new = Parameter{
                    value: syntax_buffer.clone(),
                    var_type: String::from(parameter_determine(syntax_buffer.clone(), force_string))
                };
                node_new.args.push(parameter_new);
            }
            let last_ind = nodestack.len() - 1;
			if nodestack[last_ind].borrow_mut().value == "SCOPE_GLOBAL"
			{
				let temp_str = nodestack[last_ind].borrow_mut().scope.keys().len().to_string();
				nodestack[last_ind].borrow_mut().scope.insert(temp_str.clone(), Rc::new(RefCell::new(node_new.clone())));
				nodestack.push(Rc::clone(&nodestack[last_ind].clone().borrow().scope[&temp_str]));
			}
			else
			{
				nodestack[last_ind].borrow_mut().scope.insert(node_new.value.clone(), Rc::new(RefCell::new(node_new.clone())));
				nodestack.push(Rc::clone(&nodestack[last_ind].clone().borrow().scope[&node_new.value]));
			}
            syntax_buffer.clear();
        }
        }
    }
	Rc::clone(&nodestack[0])
}