use std::fmt::format;


fn convert_io_error(err: std::io::Error) -> String {
    format!("Got an io error: \"{}\"", err)
}

fn read_count() -> Result<usize, String> {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .map_err(convert_io_error)
        .and(line.trim().parse()
             .map_err(|_| format!("Expected unsigned integer on first line, found: \"{}\".",
				  line.trim())))
}

fn read_first_empty_line() -> Result<(), String> {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .map_err(convert_io_error)
        .and(match line.trim() {
            clean if clean.is_empty() => Ok(()),
            _ => Err("Expected empty line after first.".to_string())
        })
}

enum State {
    Enter,
    Exit,
}

impl TryFrom<&str> for State {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
	type Error = String;

	match value {
	    "enter" => Ok(Self::Enter),
	    "exit" => Ok(Self::Exit),
	    _ => Err(format!("Unrecognized state \"{}\".", value)),
	}
    }
}

struct Time {
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl Time {
    fn satisfies_requirements(values: &Vec<u8>) -> bool {
	(1..=12).contains(&values[0])
	    && (1..=31).contains(&values[1])
            && (0..24).contains(&values[2])
	    && (0..60).contains(&values[3])
    }
}

impl TryFrom<&str> for Time {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
	match value.split(':').map(|v| v.parse()).collect::<Result<Vec<_>, _>>() {
	    Ok(values) if values.len() == 4 && Self::satisfies_requirements(&values)
		=> Ok(Self { month: values[0], day: values[1], hour: values[2], minute: values[3], }),
	    Ok(values) if values.len() == 4=> Err("Requremenents not satisfied.".into()),
	    Ok(values) => Err(format!("Expected 4 arguments for time, found: {}", values.len())),
	    Err(e) => Err(format!("Parsing error on time: {}", e)),
	}
    }
}

struct Input {
    uid: u64,
    time: Time,
    state: State,
    km: u64,
}

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
	let fields = value
	    .split_whitespace()
	    .collect::<Vec<_>>();

	if fields.len() != 4 {
	    return Err(format!("Expected 4 whitespace-splited fields, found {}.", fields.len()));
	}	

	Ok(Self {
	    uid: u64::from_str_radix(fields[0], 36)
		.map_err(|e| format!("Parse car uid error: {}", e))?,
	    time: fields[1].try_into()?,
	    state: fields[2].try_into()?,
	    km: fields[3].parse().map_err(|e| format!("Parse km error: {}.", e))?
	})
    }
}

fn solve_from_stdin() -> Result<(), String> {
    
}

fn main() {
    match solve_from_stdin() {
	Ok(_) => println!(),
	Err(e) => eprintln!("{}", e),
    }
}
