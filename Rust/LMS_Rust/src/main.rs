use LibraryManagementSystem::line_parser;
use LibraryManagementSystem::db_config;
use std::fs::File;
use std::fs::read_dir;
use std::io::prelude::*;
use std::env;
use std::error::Error;

fn main() {
	let args: Vec<String> = env::args().collect();
	
    println!("Library Management System\n");

	//let exe_path = env::current_exe().unwrap();
	//let current_path = env::current_dir().unwrap();
	
    let default_filename=String::from("resources/setup.xml");
	
    let config = db_config::DB_Config::db_config(if args.len()>1 && !args[1].is_empty() {
		args[1].clone()
	} else {
		default_filename
	});
	
println!("Reading from: {}", config.filename);

    let mut file = match File::open(config.filename) {	
		Ok(open_file) => open_file,
		Err(_) => panic!("\n\nInventory file not found.\n\n"),
	};

	let field_headers=config.fields;
	let parser=line_parser::LineParser::new_parser(field_headers,config.delimiter);

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input_lines:Vec<&str>=contents.split_terminator('\n').collect();
    
    let mut parsed_lines=Vec::new();
    
    for input_line in input_lines {
		parsed_lines.push(parser.parse_line(input_line));
	}

	for parsed_line in parsed_lines {
		let mut i=0;
		if valid_records(&parsed_line) {
			for field in parsed_line {
				if field!="" {
					println!("{}: {}",parser.fields[i],field);
				}
				i+=1;
			}
		}
		else {
			println!("Invalid record found");
		}
		println!("-----");
	}
}

//fn db_config(filename: String) -> DB_Config {
	//DB_Config{filename}
//}

fn valid_records(input: &Vec<&str>) -> bool {
	if input[0]!="" && input[input.len()-1]!="" {true} else {false}
}
