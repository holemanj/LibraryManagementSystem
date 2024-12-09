use std::{fs::File, io::Read};
use std::str;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

pub struct field_data {
	fieldname: String,
	is_required: bool,
}

impl field_data {
	fn new(fieldname: String, is_required: bool) -> field_data {
		field_data{fieldname,is_required}
	}
	
	pub fn new_required(fieldname: String) -> field_data {
		field_data::new(fieldname,true)
	}
	
	pub fn new_notrequired(fieldname: String) -> field_data {
		field_data::new(fieldname, false)
	}
	
	fn is_required(&self) -> bool {
		self.is_required
	}
}

pub struct DB_Config {
	pub filename: String,
	pub fields: Vec<String>,
	pub delimiter: char,
}


impl DB_Config {
	pub fn new(filename: String,fields:Vec<String>,delimiter:char) -> Self {
		DB_Config{filename,fields,delimiter}
	}

	pub fn db_config(setup_file: String) -> Self {
		let defns:LMS_definition = parse_setup(setup_file);
		DB_Config{filename:format!("{}/{}",defns.defaultPath,&defns.defaultFile),
			fields:defns.structure.field.into_iter().map(|this_field| this_field.fieldname.to_string()).collect(),
			delimiter: defns.structure.delimiter}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LMS_definition {
	defaultFile: String,
	defaultPath: String,
	structure: LMS_structure,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LMS_structure {
	delimiter: char,
	field: Vec<LMS_field>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LMS_field {
	required: bool,
	#[serde(rename = "$value")]
	fieldname: String,
}
	
fn parse_setup(setup_file: String) -> LMS_definition {
	let error_file = setup_file.clone();
	let mut setup = match File::open(setup_file) {	
		Ok(open_file) => open_file,
		Err(_) => panic!("\n\nSetup file {} not found.\n\n",error_file),
	};

	let mut data:String=String::new();
	setup.read_to_string(&mut data).unwrap();

	//return XML parsed into LMS_definition structure
	from_str(&data).unwrap()
}
