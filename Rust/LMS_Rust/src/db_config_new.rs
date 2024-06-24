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

pub struct DB_Config <'a>{
	pub filename: String,
	pub fields: Vec<&'a str>,
	pub delimiter: char,
}

//use std::fs;
//use minidom::Element;

impl <'b> DB_Config<'b> {
	pub fn new(filename: String,fields:Vec<&'b str>,delimiter:char) -> Self {
		DB_Config{filename,fields,delimiter}
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
}
