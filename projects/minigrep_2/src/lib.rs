use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        let mut case_sensitive: bool = false;

        let cs_arg = String::from("-cs");

        if args.len() > 4 {
            return Err("Too many arguments");
        } else if args.len() < 3 {
            return Err("Too few arguments");
        }

        // Set parameters
        if args.contains(&cs_arg) {
            case_sensitive = true;
        }

        // 3 arguments supplied
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_sensitive(config.query, &contents)
    } else {
        search_case_insensitive(config.query, &contents)
    };

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

/// Case-sensitive search
fn search_case_sensitive(query: String, contents: &str) -> Vec<&str> {
    let mut found_lines: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(&query) {
            found_lines.push(line);
        }
    }

    found_lines
}

/// Case-insensitive search
fn search_case_insensitive(query: String, contents: &str) -> Vec<&str> {
    let mut found_lines: Vec<&str> = vec![];
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            found_lines.push(line);
        }
    }

    found_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    mod case_insensitive {
        use super::*;

        #[test]
        fn correct_search_includes() {
            let query = "hello".to_string();
            let contents = "hello this
is a message
to say hello
as a message.";

            assert_eq!(
                search_case_insensitive(query, contents),
                vec!["hello this", "to say hello"]
            )
        }

        #[test]
        fn correct_search_not_includes() {
            let query = "saay".to_string();
            let contents = "hello this
is a message
to say hello
as a message.";
            let empty_vec: Vec<&str> = vec![];

            assert_eq!(search_case_insensitive(query, contents), empty_vec)
        }
    }

    mod case_sensitive {
        use super::*;

        #[test]
        fn correct_search_includes() {
            let query = "meSsAge".to_string();
            let contents = "hello this
is a meSsAge
to say hello
as a message.";

            assert_eq!(search_case_sensitive(query, contents), vec!["is a meSsAge"])
        }

        #[test]
        fn correct_search_not_includes() {
            let query = "SaAy".to_string();
            let contents = "hello this
is a message
to say hello
as a message.";
            let empty_vec: Vec<&str> = vec![];

            assert_eq!(search_case_sensitive(query, contents), empty_vec)
        }
    }
}
