use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Eq, PartialEq, Default)]
pub enum StructureType {
	#[default]
	Single = 0,
	Array = 1,
	Map = 2
}

#[derive(Clone, Default)]
pub struct JsonNode {
	pub value: Box<String>,
	pub single: Vec<Rc<RefCell<JsonNode>>>,
	pub map: Box<HashMap<String, Rc<RefCell<JsonNode>>>>,
	pub array: Box<Vec<Rc<RefCell<JsonNode>>>>,
	pub structure: Box<StructureType>
}

pub fn deserialize(raw_json:String) -> JsonNode {
	let mut global_node = JsonNode::default();
	*global_node.value = String::from("GLOBAL_JSON_NODE");
	//character bools and character buffers
	let mut char_buff = String::new();
	let mut last_char = 'z';
	let mut is_quoting = Box::new(false);
	let mut is_primitive = Box::new(true);
	//node stack
	let mut node_stack:Vec<Rc<RefCell<JsonNode>>> = Vec::new();
	node_stack.push(Rc::new(RefCell::new(global_node)));
	let mut morph_node = JsonNode::default();
	//tokenizer
	for raw_char in raw_json.chars() {
		if raw_char == '"'{
			*is_quoting = !*is_quoting;
		}
		if !*is_quoting {
			match raw_char {
				'{' => {
					*morph_node.structure = StructureType::Map;
					if *node_stack[node_stack.len() - 1].borrow().value == "GLOBAL_JSON_NODE"
					{
						let tmp_node = Rc::new(RefCell::new(morph_node.clone()));
						node_stack[node_stack.len() - 1].borrow_mut().map.insert(*morph_node.value.clone(), Rc::clone(&tmp_node));
						node_stack.push(Rc::clone(&tmp_node));
					}
					else
					{
						insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
					}
					morph_node = JsonNode::default();
				},
				'}' => {
					if *is_primitive {
						*morph_node.value = char_buff.clone();
						*morph_node.structure = StructureType::Single;
						insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
						*is_primitive = false;
						node_stack.pop();
					}
					else
					{
						insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
						node_stack.pop();
					}
					node_stack.pop();
					morph_node = JsonNode::default();
					char_buff = String::new();
				},
				'[' => {
					*morph_node.structure = StructureType::Array;
					if *node_stack[node_stack.len() - 1].borrow().value == "GLOBAL_JSON_NODE"
					{
						let tmp_node = Rc::new(RefCell::new(morph_node.clone()));
						node_stack[node_stack.len() - 1].borrow_mut().map.insert(*morph_node.value.clone(), Rc::clone(&tmp_node));
						node_stack.push(Rc::clone(&tmp_node));
					}
					else
					{
						insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
					}
					morph_node = JsonNode::default();
					*is_primitive = true;
				},
				']' => {
					if *is_primitive {
						*morph_node.value = char_buff.clone();
						*morph_node.structure = StructureType::Single;
						insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
						*is_primitive = false;
						node_stack.pop();
					}
					else
					{
						insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
						node_stack.pop();
					}
					node_stack.pop();
					morph_node = JsonNode::default();
					char_buff = String::new();
					*is_primitive = true;
				},
				'"' => {

				},
				',' => {
					if last_char != ']' && last_char != '}'
					{
						if *is_primitive {
							*morph_node.value = char_buff.clone();
							*morph_node.structure = StructureType::Single;
							insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
							*is_primitive = false;
							node_stack.pop();
						}
						else
						{
							insert_into_parent(&Rc::new(RefCell::new(morph_node.clone())), &mut node_stack);
							node_stack.pop();
						}
						morph_node = JsonNode::default();
						char_buff = String::new();
						*is_primitive = true;
					}
				},
				':' => {
					*morph_node.value = char_buff.clone();
					*morph_node.structure = StructureType::Single;
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
	let ret_value = node_stack[0].borrow().map[""].borrow().clone();
	ret_value
}

fn insert_into_parent(child:&Rc<RefCell<JsonNode>>, node_stack:&mut Vec<Rc<RefCell<JsonNode>>>) {
	if *node_stack[node_stack.len() - 1].borrow().structure == StructureType::Array {
		node_stack[node_stack.len() - 1].borrow_mut().array.push(Rc::clone(child));
		node_stack.push(Rc::clone(child));
	}
	else if *node_stack[node_stack.len() - 1].borrow().structure == StructureType::Map {
		node_stack[node_stack.len() - 1].borrow_mut().map.insert(*child.borrow().value.clone(), Rc::clone(child));
		node_stack.push(Rc::clone(child));
	}
	else {//single
		node_stack[node_stack.len() - 1].borrow_mut().single = vec![Rc::clone(child)];
	}
}

fn dbg_print_map(map: &Box<HashMap<String, Rc<RefCell<JsonNode>>>>) {
    for (key, value) in map.iter() {
        println!("{} / {}", key, value.borrow().value);
    }
}

fn dbg_print_array(vector: &Box<Vec<Rc<RefCell<JsonNode>>>>) {
    for (key, value) in vector.iter().enumerate() {
        println!("{} / {}", key, value.borrow().value);
    }
}