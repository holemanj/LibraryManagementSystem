pub struct DB_Config {
	pub filename: String,
}

impl DB_Config {
	pub fn db_config(filename: String) -> DB_Config {
		DB_Config{filename}
	}
}
