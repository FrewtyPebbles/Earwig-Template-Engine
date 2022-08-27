use std::process;

pub enum InvalidTemplate {
	Lowercase,
	NotRenderable,
	NoPresets,
	PresetDoesntExist
}

pub enum ErrorReason {
	None,
	InvalidTemplate(InvalidTemplate)
}

pub enum ErrorDescription {
	MissingCharacter,
	InvalidType,
	UnclosedString, //Check if string reaches \n then throw error
	InvalidTemplate
}

pub enum ErrorType {
	Syntax,
	Grammar,
	Fatal
}

fn handle_error_reason(reason:ErrorReason, hint:&mut String) -> String {
	let mut ret_val = String::new();
	match reason {
		ErrorReason::InvalidTemplate(InvalidTemplate::Lowercase) => {
			ret_val = format!("\n│   REASON: Template keywords must be uppercase.");
			*hint = String::from("\n│ HINT : Capitalize all language defined template keywords.");
		},
		ErrorReason::InvalidTemplate(InvalidTemplate::NotRenderable) => {
			ret_val = format!("\n│   REASON: This is not a renderable template.");
			*hint = String::from("\n│ HINT : Check the documentation for a list of renderable template keywords.");
		},
		ErrorReason::InvalidTemplate(InvalidTemplate::NoPresets) => {
			ret_val = format!("\n│   REASON: No PRESETs were found in the engine's earData.json file.");
			*hint = String::from("\n│ HINT : Before trying to render a preset, you must create that preset with the \"PRESET:\" template's child template \"NEW_PRESETS:\".");
		},
		ErrorReason::InvalidTemplate(InvalidTemplate::PresetDoesntExist) => {
			ret_val = format!("\n│   REASON: The PRESET in your query does not exist in the engine's earData.json file.");
			*hint = String::from("\n│ HINT : Before trying to render a preset, you must create that preset with the \"PRESET:\" template's child template \"NEW_PRESETS:\".");
		},
		ErrorReason::None => {
			if *hint != ""
			{
				*hint = format!("\n│ HINT : {}", *hint);
			}
		}
	}
	ret_val
}

pub fn handle_error(err_desc:((ErrorDescription, ErrorReason), Vec<&str>), err_sudgestion:&str, err_type:ErrorType, err_line:&str, err_column:&str, err_file:&str){
	let description;
	let mut suggestion:String = String::from(err_sudgestion);
	//DESCRIPTION
	match err_desc.0.0 {
		ErrorDescription::MissingCharacter => {
			description = format!("Expected character: '{}'\n│   AT: {}{}", err_desc.1[0], err_desc.1[1], handle_error_reason(err_desc.0.1, &mut suggestion));
		},
		ErrorDescription::InvalidType => {
			description = format!("Type is invalid: \"{}\"{}", err_desc.1[0], handle_error_reason(err_desc.0.1, &mut suggestion));
		},
		ErrorDescription::UnclosedString => {
			description = format!("Unclosed string: \"{}...{}", err_desc.1[0], handle_error_reason(err_desc.0.1, &mut suggestion));
		},
		ErrorDescription::InvalidTemplate => {
			description = format!("The Template provided is invalid:\n│\n│   TEMPLATE: \"{}\"\n│{}", err_desc.1[0], handle_error_reason(err_desc.0.1, &mut suggestion));
		}
	}
	//TYPE
	let kind_of_error = match err_type {
		ErrorType::Grammar => "Grammar",
		ErrorType::Syntax => "Syntax",
		ErrorType::Fatal => "FATAL",
	};
	eprintln!("\n┍━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╍┅┈┈\n▼ ERROR ( {} ) : {} | {} !\n● ━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━⚋━╍┅┈┈\n│  ⟲ {}\n│\n│ ▻  {}\n│{}\n┕━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╍┅┈┈\n", kind_of_error, err_line, err_column, err_file, description, suggestion);
	match err_type {
		ErrorType::Fatal => {
			process::exit(1);
		},
		_ => {
			//do nothing
		}
	}
}