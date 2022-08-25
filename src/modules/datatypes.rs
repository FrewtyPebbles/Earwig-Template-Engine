use crate::modules::parserutilities::{parse_type, node_to_json_preset, parameter_determine};
use crate::modules::parser::parse_source;
use crate::modules::jsondeserializer::{deserialize as json_deserialize};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::path::{PathBuf, Path};
use std::io::{ BufReader, Write};
use std::env;
use std::fs::OpenOptions;
use std::fs;

#[derive(Clone, Default)]
pub struct Parameter {
	pub value: String,
	pub var_type: String
}

//make parameters Rc value

#[derive(Clone, Default)]
pub struct Node {
	pub value: String,
	pub render: bool,
	pub tab_number: i32,
	pub scope: Box<HashMap<String, Rc<RefCell<Node>>>>,
	pub args: Box<Vec<Box<Parameter>>>
}

impl Node{
	pub fn call(&self, is_preset:bool) -> Node
	{
		let ret_val:Node = Node {
			value: String::from(self.value.as_str()),
			render: self.render,
			tab_number: self.tab_number,
			scope: self.scope.clone(),
			args: self.args.clone()
		};
		//println!("Parsing : {}", self.value);
		let matchkey = self.value.as_str();
		//println!("PARSING TREE {}", self.value.clone());
		match matchkey {
			"INSERT" => {
				if self.render && !is_preset {
					let node_global = Node {
						value: String::from("SCOPE_GLOBAL"),
						render: false,
						tab_number: -1,
						scope: Box::new(HashMap::from([])),
						args:  Box::new(vec![])
					};
					let cli_args: Vec<String> = env::args().collect();
					let path_string = self.scope["PATH"].borrow().args[0].value.clone();
					let path_array = path_string.split("/");
					let mut generate_path = PathBuf::from(cli_args[1].as_str()).parent().unwrap().to_path_buf();
					for path_part in path_array {
						if path_part == ".."{
							let parent_path = generate_path.parent();
							if parent_path == None {
								generate_path = PathBuf::from("./").parent().unwrap().to_path_buf();
							}
							else
							{
								generate_path = parent_path.unwrap().to_path_buf();
							}
						}
						else{
							generate_path = generate_path.to_path_buf().join(path_part);
						}
					}
					//println!("PATH {}", generate_path.clone().into_os_string().into_string().unwrap());
					let file = File::open(generate_path.clone().into_os_string().into_string().unwrap())
						.expect("Failed to open .ear file.");
					let origin_reader = BufReader::new(file);
					parse_source(origin_reader, node_global).borrow_mut().interpret(is_preset);
				}
			},
			"MIME" => {
				if self.render && !is_preset {
					println!("mime_type(\"{}\");", self.args[0].value);
					std::io::stdout().flush().ok().expect("stdout failed to flush");
				}
			},
			"HEADERS" => {
				
				fn recursive_header(header:Node) -> String {
					let mut header_dict = String::new();
					header_dict += "{";
					for (header_tag, header_value) in header.scope.iter()
					{
						if header_value.borrow().scope.clone().is_empty() && header_value.borrow().args.len() > 0
						{
							header_dict += format!(" \"{}\" : {},", header_tag.as_str(), parse_type(*header_value.borrow_mut().args[0].clone()).as_str()).as_str();
						}
						else
						{
							header_dict += format!(" \"{}\" : {},", header_tag.as_str(), recursive_header(header_value.borrow_mut().clone()).as_str()).as_str();
						}
					}
					header_dict += " }";
					header_dict
				}
				println!("set_headers({});", recursive_header(self.clone()).as_str());
			},
			"REQUEST_LIMIT" => {
				if self.render && !is_preset {
					println!("request_limit({}, {});", parse_type(*self.args[0].clone()), parse_type(*self.args[2].clone()));
					std::io::stdout().flush().ok().expect("stdout failed to flush");
				}
			},
			"PRESET" => {
				if self.scope.contains_key("NEW_PRESETS")//make new key function
				{
					if !Path::new("earData.json").exists() {
						File::create("earData.json").unwrap();
					}
					let mut file = OpenOptions::new()
					.write(true)
					.truncate(true)
					.open("earData.json")
					.unwrap();
					let mut json_objects:String = String::new();
					for (_,curr_preset) in self.scope["NEW_PRESETS"].borrow().scope.clone().iter(){
						json_objects += node_to_json_preset(curr_preset.borrow().clone(), curr_preset.borrow().value.clone()).as_str();
					}
					json_objects.pop();
					if let Err(e) = write!(file, "{{{}}}", json_objects) {
						println!("Couldn't write to file: {}", e);
					}
				}
				if !self.scope.contains_key("NEW_PRESETS")
				{
					//println!("DESERIALIZING");
					let json_nodes = json_deserialize(fs::read_to_string("earData.json").expect("Failed to read earData.json."));
					for (k, _) in self.scope.iter() {
						json_nodes.scope[k].borrow_mut().interpret(false);
					}
				}
			},
			"IF" => {

			},
			"PER" => {

			},
			_ => {
				if self.render {
					println!(" < ! ERR ! > \"{}\" IS NOT A RENDERABLE KEY.", matchkey)
				}
			}
		}
		ret_val
	}
	pub fn interpret(&mut self, mut is_preset:bool) -> Node {
		/*print!("\nValue: \"{}\"\n - Tabs: {}\n", self.value, self.tab_number);
		println!(" - Scope:");
		for (scope_index_debug, scope_debug) in self.scope.iter()
		{
			println!(" {} - {}", scope_index_debug, scope_debug.borrow().value);
		}
		println!(" - Arg:");
		for (argument_index_debug, argument_debug) in self.args.iter().enumerate()
		{
			println!(" {} - {} : \"{}\"", argument_index_debug, argument_debug.var_type, argument_debug.value);
		}*/
		let mut new_scope:HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
		if self.value == "SCOPE_GLOBAL"
		{
			for current_ind in 0..self.scope.keys().len() {
				if self.scope[&current_ind.to_string()].borrow().scope.is_empty()
				{
					new_scope.insert(self.scope[&current_ind.to_string()].borrow().value.clone(), Rc::new(RefCell::new(self.scope[&current_ind.to_string()].borrow().call(is_preset))));
				}
				else
				{
					let temp_node = self.scope[&current_ind.to_string()].borrow_mut().interpret(is_preset.clone());
					if self.render {
						new_scope.insert(self.scope[&current_ind.to_string()].borrow().value.clone(), Rc::new(RefCell::new(temp_node.call(is_preset.clone()))));
					}
					else
					{
						new_scope.insert(self.scope[&current_ind.to_string()].borrow().value.clone(), Rc::new(RefCell::new(temp_node)));
					}
				}
			}
		}
		else {
			for (_, current) in self.scope.iter() {
				if current.borrow().value != "" {
					if current.borrow().value == "NEW_PRESETS"
					{
						is_preset = true;
					}
					if current.borrow().scope.is_empty()
					{
						new_scope.insert(current.borrow().value.clone(), Rc::new(RefCell::new(current.borrow().call(is_preset.clone()))));
					}
					else
					{
						let temp_node = current.borrow_mut().interpret(is_preset.clone());
						if self.render {
							new_scope.insert(current.borrow().value.clone(), Rc::new(RefCell::new(temp_node.call(is_preset.clone()))));
						}
						else
						{
							new_scope.insert(current.borrow().value.clone(), Rc::new(RefCell::new(temp_node)));
						}
					}
					if current.borrow().value == "NEW_PRESETS"
					{
						is_preset = false;
					}
				}
			}
		}
		self.scope.clear();
		self.scope = Box::new(new_scope);
		if self.render && !is_preset && self.value != ""{
			self.call(false);
		}
		self.clone()
	}
}

