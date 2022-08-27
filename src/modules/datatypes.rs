use crate::modules::parserutilities::{parse_type, node_to_json_preset};
use crate::modules::parser::parse_source;
use crate::modules::jsondeserializer::{deserialize as json_deserialize};
//Error handler
use crate::modules::errorhandler::{handle_error, ErrorReason, ErrorDescription, ErrorType, InvalidTemplate};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::path::{PathBuf, Path};
use std::io::{self, BufReader, Write, Read};
use std::env;
use std::fs::OpenOptions;
use std::fs;
use regex::Regex;

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
	pub fn call(&self, is_preset:bool, file_path:String) -> Node
	{
		let ret_val:Node = Node {
			value: String::from(self.value.as_str()),
			render: self.render,
			tab_number: self.tab_number,
			scope: self.scope.clone(),
			args: self.args.clone()
		};
		//println!("Parsing : {}", self.value.clone());
		let matchkey = self.value.as_str();
		//println!("PARSING TREE {}", self.value.clone());
		match matchkey {
			"TEXT_GLOBAL" => {
				if self.render && !is_preset {
					print!("{}", self.args[0].value);
					std::io::stdout().flush().ok().expect("stdout failed to flush");
				}
			},
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
					let mut origin_reader:BufReader<File> = BufReader::new(file);
					let mut tmp_str = String::new();
					let mut _exec_str = String::new();
					origin_reader.read_to_string(&mut tmp_str)
					.expect("Insert failed to read .ear file to string.");
					let mut re:Regex;// this regex is used to find the substitution, later implement a faster O(n) approach
					for (key, val) in self.scope["SUBSTITUTIONS"].borrow().scope.iter() {
						re = Regex::new(&format!(r"(?m)(\{{{}:).+(\}})", key).as_str()).unwrap();
						tmp_str = tmp_str.replace(format!("{{{}}}", key).as_str(), parse_type(*val.borrow().args[0].clone()).as_str());
						tmp_str = re.replace_all(tmp_str.as_str(), parse_type(*val.borrow().args[0].clone()).as_str()).to_string();
					}
					_exec_str = tmp_str;
					parse_source(format!("{}\n$",_exec_str), node_global, format!("{} ~> {}", file_path.clone(), generate_path.clone().into_os_string().into_string().unwrap())).borrow_mut().interpret(false, format!("{} ~> {}", file_path.clone(), generate_path.clone().into_os_string().into_string().unwrap()));
				}
			},
			"MIME" => {
				if self.render && !is_preset {
					print!("mime_type(\"{}\");", self.args[0].value);
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
				print!("set_headers({});", recursive_header(self.clone()).as_str());
				io::stdout().flush().expect("Couldn't flush stdout");
			},
			"REQUEST_LIMIT" => {
				if self.render && !is_preset {
					print!("request_limit({}, {});", parse_type(*self.args[0].clone()), parse_type(*self.args[2].clone()));
					std::io::stdout().flush().ok().expect("stdout failed to flush");
				}
			},
			"PRESET" => {
				if self.scope.contains_key("NEW_PRESETS")//make new key function
				{
					let mut json_objects:String = String::new();
					for (_,curr_preset) in self.scope["NEW_PRESETS"].borrow().scope.clone().iter(){
						json_objects += node_to_json_preset(curr_preset.borrow().clone(), curr_preset.borrow().value.clone()).as_str();
					}
					json_objects.pop();
					//println!("JSON OBJS {}", json_objects);
					let mut file = match File::create(Path::new("earData.json")) {
						Err(why) => panic!("{}", why),
						Ok(file) => file,
					};
					match file.write_all(format!("{{{}}}", json_objects).as_bytes()) {
						Err(why) => panic!("{}", why),
						Ok(_) => ""
					};
				}
				if !self.scope.contains_key("NEW_PRESETS")
				{
					//println!("DESERIALIZING");
					let json_nodes = json_deserialize(match fs::read_to_string("earData.json"){
						Err(_) => {
							"{}".to_string()
						},
						Ok(ret_str) => {
							if ret_str == "" {
								handle_error(
									(
										(
											ErrorDescription::InvalidTemplate,
											ErrorReason::InvalidTemplate(InvalidTemplate::NoPresets)
										),
										vec![self.value.as_str()]
									),
									"",
									ErrorType::Fatal,
									"?",
									format!("{}",self.tab_number.clone()).as_str(),
									file_path.clone().as_str()
								);
							}
							ret_str
						}
					});
					for (k, v) in self.scope.iter() {
						if !json_nodes.scope.contains_key(k) {
							handle_error(
								(
									(
										ErrorDescription::InvalidTemplate,
										ErrorReason::InvalidTemplate(InvalidTemplate::PresetDoesntExist)
									),
									vec![k]
								),
								"",
								ErrorType::Fatal,
								"?",
								format!("{}",v.borrow().tab_number.clone()).as_str(),
								file_path.clone().as_str()
							);
						}
						json_nodes.scope[k].borrow_mut().interpret(false, format!("{} -> PRESETS", file_path.clone()));
					}
				}
			},
			"IF" => {

			},
			"PER" => {

			},
			_ => {
				if self.render {
					handle_error(
						(
							(
								ErrorDescription::InvalidTemplate,
								ErrorReason::InvalidTemplate(InvalidTemplate::NotRenderable)
							),
							vec![self.value.as_str()]
						),
						"",
						ErrorType::Grammar,
						"?",
						format!("{}",self.tab_number.clone()).as_str(),
						file_path.clone().as_str()
					);
				}
			}
		}
		ret_val
	}
	pub fn interpret(&mut self, mut is_preset:bool, file_path:String) -> Node {
		let mut new_scope:HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
		if self.value == "SCOPE_GLOBAL"
		{
			for current_ind in 0..self.scope.keys().len() {
				if self.scope[&current_ind.to_string()].borrow().scope.is_empty()
				{
					new_scope.insert(self.scope[&current_ind.to_string()].borrow().value.clone(), Rc::new(RefCell::new(self.scope[&current_ind.to_string()].borrow().call(is_preset, file_path.clone()))));
				}
				else
				{
					let temp_node = self.scope[&current_ind.to_string()].borrow_mut().interpret(is_preset.clone(), file_path.clone());
					if self.render {
						new_scope.insert(self.scope[&current_ind.to_string()].borrow().value.clone(), Rc::new(RefCell::new(temp_node.call(is_preset.clone(), file_path.clone()))));
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
						new_scope.insert(current.borrow().value.clone(), Rc::new(RefCell::new(current.borrow().call(is_preset.clone(), file_path.clone()))));
					}
					else
					{
						let temp_node = current.borrow_mut().interpret(is_preset.clone(), file_path.clone());
						if self.render {
							new_scope.insert(current.borrow().value.clone(), Rc::new(RefCell::new(temp_node.call(is_preset.clone(), file_path.clone()))));
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
			self.call(false, file_path.clone());
		}
		self.clone()
	}
}

