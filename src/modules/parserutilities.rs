use crate::modules::datatypes::{Parameter, Node};

pub fn parameter_determine(raw:String, force_string:bool) -> String {
		let mut ret_val;
	
	
		if force_string {
			ret_val = String::from("str");
		}
		else {
			if raw.contains(":"){
				ret_val = String::from("time");
			}
			else if raw.chars().last().unwrap().is_numeric(){
				ret_val = String::from("number");
				for raw_char in raw.chars() {
					if !raw_char.is_numeric() && raw_char != '-' && raw_char != '.' {
						ret_val = String::from("str");
						break;
					}
				}
			}
			else if raw.to_uppercase() == raw && raw.chars().all(char::is_alphabetic)
			{
				ret_val = String::from("keyword");
			}
			else {
				ret_val = String::from("str");
			}
		}
	
		ret_val
	}
	
	pub fn parse_type(param:Parameter) -> String{
		let mut ret_val:String = String::new();
		match param.var_type.clone().as_str()
		{
			"time" => {
				let split_time:Vec<&str> = param.value.split(":").collect();
				//1hour = 3600000ms
				//1min = 60000
				//1sec = 1000
				let mut time = 0;
				if split_time.len() == 3
				{
					time += split_time[0].parse::<i32>().unwrap() * 3600000;
					time += split_time[1].parse::<i32>().unwrap() * 60000;
					time += split_time[2].parse::<i32>().unwrap() * 1000;
				}
				else if split_time.len() == 2
				{
					time += split_time[0].parse::<i32>().unwrap() * 60000;
					time += split_time[1].parse::<i32>().unwrap() * 1000;
				}
				else
				{
					time += split_time[0].parse::<i32>().unwrap() * 1000;
				}
				ret_val = format!("{}", time);
			},
			"str" => {
				ret_val = format!("\"{}\"", param.value.clone());
			},
			"number" => {
				ret_val = param.value.clone();
			},
			"keyword" => {
				ret_val = format!("\"!#!KEYWORD:{}\"",param.value.clone());
			},
			_ => {
	
			}
		}
		ret_val
	}
	
	pub fn node_to_json_preset(the_node:Node, identifier:String) -> String{
		fn recursive_json(header:Node) -> String {
			let mut header_dict = String::new();
			header_dict += "{";
			header_dict += format!("\"!#!nodeparam:tab_number\":{},", header.tab_number.clone()).as_str();
			for (header_tag, header_value) in header.scope.iter()
			{
				if header_value.borrow_mut().scope.is_empty()
				{
					header_dict += format!("\"{}\":[", header_tag.as_str()).as_str();
					for curr_arg in header_value.borrow_mut().args.iter(){
						header_dict += format!("{},", parse_type(curr_arg.clone()).as_str()).as_str();
					}
					header_dict.pop();
					header_dict += "],"
				}
				else
				{
					header_dict += format!("\"{}\":{},", header_tag.as_str(), recursive_json(header_value.borrow_mut().clone()).as_str()).as_str();
				}
			}
			header_dict.pop();
			header_dict += "}";
			header_dict
		}
		let mut ret_val = String::new();
		ret_val += format!("\"{}\":{},", identifier, recursive_json(the_node)).as_str();
		ret_val
	}
