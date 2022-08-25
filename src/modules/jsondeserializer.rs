use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::modules::datatypes::{Node, Parameter};
use crate::modules::parserutilities::parameter_determine;

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub enum StructureType {
	#[default]
	Single,
	Array,
	Map
}

#[derive(Clone, Default)]
pub struct JsonNode {
	pub value: Box<String>,
	pub single: Box<String>,
	pub map: Box<HashMap<String, Rc<RefCell<JsonNode>>>>,
	pub array: Box<Vec<Rc<RefCell<JsonNode>>>>,
	pub structure: Box<StructureType>
}

pub fn deserialize(raw_json:String) -> Node {
	let mut global_node = (Node::default(), StructureType::Map, String::new());
	global_node.0.value = String::from("GLOBAL_JSON_NODE");
	global_node.1 = StructureType::Map;
	//character bools and character buffers
	let mut char_buff = String::new();
	let mut last_char = 'z';
	let mut is_quoting = Box::new(false);
	let mut is_primitive = Box::new(true);
	//node stack
	let mut node_stack:Vec<(Rc<RefCell<Node>>, StructureType, String)> = Vec::new();
	let mut global_ptr = Rc::new(RefCell::new(global_node.0));
	node_stack.push((Rc::clone(&global_ptr), global_node.1, String::new()));
	let mut morph_node = (Node::default(), StructureType::Single, String::new());
	morph_node.0.value = String::from("");
	//tokenizer
	for raw_char in raw_json.chars() {
		if raw_char == '"'{
			*is_quoting = !*is_quoting;
		}
		if !*is_quoting {
			match raw_char {
				'{' => {
					morph_node.1 = StructureType::Map;
					if node_stack.len() == 1
					{
						let tmp_node = Rc::new(RefCell::new(morph_node.0.clone()));
						node_stack[0].0.borrow_mut().scope.insert(morph_node.0.value.clone(), Rc::clone(&tmp_node));
						node_stack.push((Rc::clone(&tmp_node), morph_node.1, String::new()));
					}
					else
					{
						insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
					}
					*is_primitive = true;
					morph_node = (Node::default(), StructureType::Single, String::new());
					morph_node.0.value = String::from("");
				},
				'}' => {
					if last_char != '}' && last_char != ']'
					{
						if *is_primitive {
							println!("1PRIMITIVE");
							morph_node.0.value = char_buff.clone();
							morph_node.1 = StructureType::Single;
							insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
							//*is_primitive = false;
							node_stack.pop();
						}
						else
						{
							morph_node.2 = char_buff.clone();
							insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
							node_stack.pop();
							//node_stack.pop();
							//*is_primitive = true;
						}
					}
					
					*is_primitive = true;
					morph_node = (Node::default(), StructureType::Single, String::new());
					morph_node.0.value = String::from("");
					char_buff = String::new();
				},
				'[' => {
					morph_node.1 = StructureType::Array;
					if node_stack[node_stack.len() - 1].0.borrow().value == "GLOBAL_JSON_NODE"
					{
						let tmp_node = Rc::new(RefCell::new(morph_node.0.clone()));
						node_stack[node_stack.len() - 1].0.borrow_mut().scope.insert(morph_node.0.value.clone(), Rc::clone(&tmp_node));
						node_stack.push((Rc::clone(&tmp_node), morph_node.1, String::new()));
					}
					else
					{
						insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
					}
					*is_primitive = true;
					morph_node = (Node::default(), StructureType::Single, String::new());
					morph_node.0.value = String::from("");
					//*is_primitive = true;
				},
				']' => {
					if last_char != '}' && last_char != ']'
					{
						if *is_primitive {
							println!("2PRIMITIVE");
							morph_node.0.value = char_buff.clone();
							morph_node.1 = StructureType::Single;
							insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
							//*is_primitive = false;
							node_stack.pop();
						}
						else
						{
							morph_node.2 = char_buff.clone();
							insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
							node_stack.pop();
							//node_stack.pop();
							//*is_primitive = true;
						}
					}
					*is_primitive = true;
					morph_node = (Node::default(), StructureType::Single, String::new());
					morph_node.0.value = String::from("");
					char_buff = String::new();
					
				},
				'"' => {

				},
				',' => {
					if last_char != ']' && last_char != '}'
					{
						if *is_primitive {
							println!("3PRIMITIVE");
							morph_node.0.value = char_buff.clone();
							morph_node.1 = StructureType::Single;
							insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
							//*is_primitive = false;
							node_stack.pop();
						}
						else
						{
							morph_node.2 = char_buff.clone();
							insert_into_parent((&Rc::new(RefCell::new(morph_node.0.clone())), morph_node.1, morph_node.2), &mut node_stack);
							node_stack.pop();
						}
					}
						*is_primitive = true;
						morph_node = (Node::default(), StructureType::Single, String::new());
						morph_node.0.value = String::from("");
						char_buff = String::new();
						
				},
				':' => {
					morph_node.0.value = char_buff.clone();
					morph_node.1 = StructureType::Single;
					*is_primitive = false;
					char_buff = String::new();
				},
				_ => {
					char_buff.push(raw_char.clone());
				}
			}
		}
		else if !(raw_char == '"' && last_char != '\\') {
			char_buff.push(raw_char.clone());
		}
		last_char = raw_char.clone();
	}
	dbg_print_map(&global_ptr.borrow().scope[""].borrow().scope.clone());
	let ret_value = &global_ptr.borrow().scope[""].borrow().clone();
	ret_value.clone()
}

fn insert_into_parent(child:(&Rc<RefCell<Node>>, StructureType, String), node_stack:&mut Vec<(Rc<RefCell<Node>>, StructureType, String)>) {
	println!("DESERIALIZE0 , {}", node_stack[node_stack.len() - 1].0.borrow().value);
	println!("DESERIALIZE1 , {:?}", node_stack[node_stack.len() - 1].1);
	println!("DESERIALIZE2 , {}", node_stack[node_stack.len() - 1].2);
	{
		if child.1 == StructureType::Single {//single
			if child.0.borrow().value == "!#!RENDER" {
				node_stack[node_stack.len() - 1].0.borrow_mut().render = if child.2 == "true" { true } else { false };
				println!("3{}", node_stack[node_stack.len() - 1].0.borrow_mut().render)
			}
		}
		else if node_stack[node_stack.len() - 1].1 == StructureType::Map {
			if child.0.borrow().value == "!#!RENDER" {
				println!("2{}", node_stack[node_stack.len() - 1].0.borrow_mut().render)
			}
			node_stack[node_stack.len() - 1].0.borrow_mut().scope.insert(child.0.borrow().value.clone(), Rc::clone(child.0));
			node_stack.push((Rc::clone(child.0), child.1, String::new()));
		}
		else if node_stack[node_stack.len() - 1].1 == StructureType::Array {
			if child.0.borrow().value == "!#!RENDER" {
				println!("1{}", node_stack[node_stack.len() - 1].0.borrow_mut().render)
			}
			node_stack[node_stack.len() - 1].0.borrow_mut().args.push(Box::new(Parameter {
				value: child.0.borrow().value.clone(),
				var_type: parameter_determine(child.0.borrow().value.clone(), false),
			}));
			node_stack.push((Rc::clone(child.0), child.1, String::new()));
		}
		
	}
}

fn dbg_print_map(map: &Box<HashMap<String, Rc<RefCell<Node>>>>) {
    for (key, value) in map.iter() {
        println!("{} / {} ({})", key, value.borrow().value, value.borrow().render);
    }
}

fn dbg_print_array(vector: &Box<Vec<Rc<RefCell<JsonNode>>>>) {
    for (key, value) in vector.iter().enumerate() {
        println!("{} / {}", key, value.borrow().value);
    }
}