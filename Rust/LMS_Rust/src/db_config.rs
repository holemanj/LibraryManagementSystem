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
		let defns:LMS_definition = parse_setup();
		DB_Config{filename:defns.defaultFile, 
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
	
fn parse_setup() -> LMS_definition {
		let data: &'static str = r##"
<LMS xmlns="LMS">
	<defaultFile>inventory.txt</defaultFile>
	<defaultPath>./resources</defaultPath>
	<structure delimiter="|">
		<field required="1">Title</field>
		<field required="0">Author(s)</field>
		<field required="0">Publisher</field>
		<field required="0">Year</field>
		<field required="1">Count</field>
	</structure>
</LMS>	"##;

	from_str(data).unwrap()
}


/*	pub fn db_config(setup_file: String) -> Self {
		println!("Reading from {:?}",setup_file);
		let xmlData: String = fs::read_to_string(setup_file).unwrap();
		let root: Element = xmlData.parse().unwrap();
		
		let mut read_filename: String = "".to_string();
		let mut read_delimiter: char = '|';
		let mut read_fields: Vec<&str> = Vec::new();
		
		for e in root.children() {
			match e.name() {
				"defaultFile" =>  read_filename = e.text(),
				"defaultPath" => println!("Default filepath: {:?}",e.text()),
				"structure" => {
					read_delimiter = e.attr("delimiter").unwrap().chars().next().unwrap();
					println!("Data fields:");
					for field in e.children() {
						if field.attr("required").unwrap() == "1" {
							print!("* ");
							read_fields.push(&field.text());
							//read_fields.push(field_data::new_required(field.text()));
						} else {
							//read_fields.push(field_data::new_notrequired(field.text()));
							read_fields.push(&field.text());
						}
						println!("{:?}", field.text());
					}
				}
				_=>{println!("{:?}",e.name())}
			}
		}
		DB_Config{filename:read_filename, fields:read_fields, delimiter:read_delimiter}
	}*/

