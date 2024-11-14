use std::{error::Error, fs, env};


pub struct Config {
    pub query: String,
    pub file_path: String,
	pub ignore_case: bool,
}

impl Config {
    pub fn build(
		mut args: impl Iterator<Item = String>
	) -> Result<Config, &'static str> {

		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string"),
		};

		let file_path = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file path"),
		};

		let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines().filter(|s| s.contains(query)).collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut result = vec![];

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			result.push(line);
		}
	}
	result
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

	let result = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};

    for line in result {
		println!("{line}");
	}
	Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents))
	}
}






