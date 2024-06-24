pub struct LineParser<'a> {
	pub fields: Vec<&'a str>,
	delimiter: char,
}

impl LineParser <'_> {
	pub fn new_parser(field_list: Vec<&str>, delimiter_choice: char) -> LineParser {
		LineParser {
			fields: field_list,
			delimiter: delimiter_choice,
		}
	}
	
	pub fn parse_line<'a>(&self,line: &'a str) -> Vec<&'a str> {
		line.split_terminator(self.delimiter).collect()
	}
}
