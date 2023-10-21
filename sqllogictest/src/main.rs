use std::error::Error;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Clone)]
struct ColumnType(char);

impl sqllogictest::ColumnType for ColumnType {
    fn from_char(value: char) -> Option<Self> {
        Some(Self(value))
    }

    fn to_char(&self) -> char {
        self.0
    }
}

// random script to dump all the sql in sqllogictest file
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut explain = false;
    match &args[..] {
        [_] => {}
        [_, arg] if arg == "--explain" => explain = true,
        _ => {
            eprintln!("Usage: {} [--explain]", args[0]);
            std::process::exit(1);
        }
    }

    let mut script = String::new();
    std::io::stdin().read_to_string(&mut script)?;
    let records = sqllogictest::parse::<ColumnType>(&script)?;
    for record in records {
        match record {
            sqllogictest::Record::Statement { sql, .. }
            | sqllogictest::Record::Query { sql, .. } => {
                // don't add explain to write statements
                if explain && (sql.starts_with("SELECT") || sql.starts_with("WITH")) {
                    println!("EXPLAIN {sql};");
                } else {
                    println!("{sql};");
                }
            }
            _ => {}
        }
    }
    Ok(())
}
