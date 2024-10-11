pub struct LineParser {
	pub fields: Vec<String>,
	delimiter: char,
}

impl LineParser {
	pub fn new_parser(field_list: Vec<String>, delimiter_choice: char) -> LineParser {
		LineParser {
			fields: field_list,
			delimiter: delimiter_choice,
		}
	}
	
	pub fn parse_line<'a>(&self,line: &'a str) -> Vec<&'a str> {
		line.split_terminator(self.delimiter).collect()
	}
}
