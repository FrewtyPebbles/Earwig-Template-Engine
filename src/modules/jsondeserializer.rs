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
	let mut has_value = Box::new(false);
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
					println!("VAL: {}", node_stack[node_stack.len() - 1].borrow_mut().value.clone());
					morph_node.structure = Box::new(StructureType::Map);
					let node_cpy = Rc::new(RefCell::new(morph_node.clone()));
					if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Map) {
						node_stack[node_stack.len() - 1].borrow_mut().map.insert(*node_cpy.borrow().value.clone(), Rc::clone(&node_cpy));
					}
					else if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Array) {
						node_stack[node_stack.len() - 1].borrow_mut().array.push(Rc::clone(&node_cpy));
					}
					else {
						node_stack[node_stack.len() - 1].borrow_mut().single = vec![Rc::clone(&node_cpy)];
					}
					node_stack.push(Rc::clone(&node_cpy));
					//clr the morph node
					*morph_node.structure = StructureType::Single;
					*morph_node.value = String::new();
					*morph_node.array = Vec::new();
					*morph_node.map = HashMap::new();
				},
				'}' => {
					let mut temp_node = JsonNode::default();
					*temp_node.value = char_buff.clone();
					morph_node.single = vec![Rc::new(RefCell::new(temp_node))];
					if *has_value {
						morph_node.structure = Box::new(StructureType::Single);
						let node_cpy = Rc::new(RefCell::new(morph_node.clone()));
						if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Map) {
							node_stack[node_stack.len() - 1].borrow_mut().map.insert(*node_cpy.borrow().value.clone(), Rc::clone(&node_cpy));
						}
						else if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Array) {
							node_stack[node_stack.len() - 1].borrow_mut().array.push(Rc::clone(&node_cpy));
						}
						else {
							node_stack[node_stack.len() - 1].borrow_mut().single = vec![Rc::clone(&node_cpy)];
						}
						node_stack.push(Rc::clone(&node_cpy));
						//clr the morph node
						*morph_node.structure = StructureType::Single;
						*morph_node.value = String::new();
						*morph_node.array = Vec::new();
						*morph_node.map = HashMap::new();
					}
					println!("{} : {}", morph_node.value.clone(), char_buff.clone());
					node_stack.pop();
					*has_value = false;
				},
				'[' => {
					morph_node.structure = Box::new(StructureType::Array);
					let node_cpy = Rc::new(RefCell::new(morph_node.clone()));
					if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Map) {
						node_stack[node_stack.len() - 1].borrow_mut().map.insert(*node_cpy.borrow().value.clone(), Rc::clone(&node_cpy));
					}
					else if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Array){
						node_stack[node_stack.len() - 1].borrow_mut().array.push(Rc::clone(&node_cpy));
					}
					else {
						node_stack[node_stack.len() - 1].borrow_mut().single = vec![Rc::clone(&node_cpy)];
					}
					node_stack.push(Rc::clone(&node_cpy));
					//clr the morph node
					*morph_node.structure = StructureType::Single;
					*morph_node.value = String::new();
					*morph_node.array = Vec::new();
					*morph_node.map = HashMap::new();
				},
				']' => {
					let mut temp_node = JsonNode::default();
					*temp_node.value = char_buff.clone();
					morph_node.single = vec![Rc::new(RefCell::new(temp_node.clone()))];
					if *has_value {
						morph_node.structure = Box::new(StructureType::Single);
						let node_cpy = Rc::new(RefCell::new(morph_node.clone()));
						if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Map) {
							node_stack[node_stack.len() - 1].borrow_mut().map.insert(*node_cpy.borrow().value.clone(), Rc::clone(&node_cpy));
						}
						else if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Array) {
							node_stack[node_stack.len() - 1].borrow_mut().array.push(Rc::clone(&node_cpy));
						}
						else {
							node_stack[node_stack.len() - 1].borrow_mut().single = vec![Rc::clone(&node_cpy)];
						}
						node_stack.push(Rc::clone(&node_cpy));
						//clr the morph node
						*morph_node.structure = StructureType::Single;
						*morph_node.value = String::new();
						*morph_node.array = Vec::new();
						*morph_node.map = HashMap::new();
					}
					println!("{} : {}", morph_node.value.clone(), char_buff.clone());
					node_stack.pop();
					*has_value = false;
				},
				'"' => {

				},
				',' => {
					let mut temp_node = JsonNode::default();
					*temp_node.value = char_buff.clone();
					morph_node.single = vec![Rc::new(RefCell::new(temp_node.clone()))];
					if *has_value {
						morph_node.structure = Box::new(StructureType::Single);
						let node_cpy = Rc::new(RefCell::new(morph_node.clone()));
						if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Map) {
							node_stack[node_stack.len() - 1].borrow_mut().map.insert(*node_cpy.borrow().value.clone(), Rc::clone(&node_cpy));
						}
						else if node_stack[node_stack.len() - 1].borrow_mut().structure == Box::new(StructureType::Array) {
							node_stack[node_stack.len() - 1].borrow_mut().array.push(Rc::clone(&node_cpy));
						}
						else {
							node_stack[node_stack.len() - 1].borrow_mut().single = vec![Rc::clone(&node_cpy)];
						}
						node_stack.push(Rc::clone(&node_cpy));
						//clr the morph node
						*morph_node.structure = StructureType::Single;
						*morph_node.value = String::new();
						*morph_node.array = Vec::new();
						*morph_node.map = HashMap::new();
					}
					char_buff = String::from("");
					*has_value = false;
				},
				':' => {
					*morph_node.value = char_buff.clone();
					println!("VALUE: {}", char_buff.clone());
					char_buff = String::from("");
					*has_value = true;
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
	dbg_print_map(&node_stack[0].borrow().map);
	let ret_value = node_stack[0].borrow().map[""].borrow().clone();
	ret_value
}

fn dbg_print_map(map: &Box<HashMap<String, Rc<RefCell<JsonNode>>>>) {
    for (key, value) in map.iter() {
        println!("{} / {}", key, value.borrow().value);
    }
}