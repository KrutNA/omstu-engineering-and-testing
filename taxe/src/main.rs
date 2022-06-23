use std::{hash::{Hash, Hasher}, collections::{HashSet, HashMap}, ops::Sub};


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

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Enter,
    Exit,
}

impl TryFrom<&str> for State {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
	match value {
	    "enter" => Ok(Self::Enter),
	    "exit" => Ok(Self::Exit),
	    _ => Err(format!("Unrecognized state \"{}\".", value)),
	}
    }
}

#[derive(Clone, Copy)]
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

impl Sub for Time {
    type Output = u64;

    fn sub(self, other: Self) -> Self::Output {
	(self.day - other.day) as u64 * 24
	    + (self.hour - other.hour) as u64
	    + if (self.minute as i16 - other.minute as i16) > 0 { 1 } else { 0 }
    }
}

#[derive(Clone, Copy)]
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

impl Hash for Input {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
	self.uid == other.uid
    }
}

impl Eq for Input {}

fn solve_block(block: &Vec<String>) -> Result<(), String> {
    let prices = &block[0];
    let prices = prices
	.split_whitespace()
	.map(|v| v.parse::<u64>())
	.collect::<Result<Vec<_>, _>>()
	.map_err(|e| format!("couldn't parse number: {}", e))?;

    if prices.len() != 0 {
	return Err(format!("expected 24 arguments, got: {}", prices.len()))
    }
    
    let inputs = block.iter()
        .enumerate()
        .skip(1)
	.map(|(line_idx, line)| line
	     .as_str()
	     .try_into()
	     .map_err(|e| format!("on line {}: {}", line_idx, e)))
	.collect::<Result<Vec<Input>, String>>()?;

    let mut overall_price: HashMap<u64, u64> = HashMap::new();
    let mut last_infos: HashSet<Input> = HashSet::new();

    for (line_idx, input) in inputs.into_iter().enumerate() {
	if let Some(info) = last_infos.get(&input) {
	    if info.state != input.state {
		let n = input.time - info.time;
		let entry = overall_price.entry(info.uid).or_insert(0);
		*entry += (info.time.hour as u64 ..=(info.time.hour as u64 + n))
		    .map(|v| prices[v as usize % 24])
		    .sum::<u64>();
	    } else {
		return Err(format!("on line {} unexpected enter state.", line_idx));
	    }
	} else if input.state == State::Enter {
	    last_infos.insert(input);
	} else {
	    return Err(format!("on line {} unexpected exit state.", line_idx));
	}
    }

    for elt in overall_price.iter() {
	println!("{} ${}", elt.0, *elt.1 as f64 / 100.0);
    }

    Ok(())
}

fn read_block_from_stdin(block_idx: usize) -> Result<Vec<String>, String> {
    let mut lines = vec![];

    for line_idx in 1.. {
	let mut line = String::new();
	std::io::stdin()
	    .read_line(&mut line)
	    .map_err(|e| format!("Io error on block {}, line {}: {}",
				 block_idx, line_idx, e)
	    )?;
	line = line.trim().into();
	if line.len() == 0 { break; }
	lines.push(line);
    }

    Ok(lines)
}

fn solve_from_stdin() -> Result<(), String> {
    let count = read_count()?;
    read_first_empty_line()?;

    let blocks = (1..count)
	.map(read_block_from_stdin)
	.collect::<Result<Vec<_>, _>>()?;

    for (block_idx, block) in blocks.iter().enumerate() {
	match solve_block(block) {
	    Err(e) => eprintln!("Got an error on block {} {}", block_idx + 1, e),
	    _ => (),
	}
    }

    Ok(())
}

fn main() {
    match solve_from_stdin() {
	Ok(_) => println!(),
	Err(e) => eprintln!("{}", e),
    }
}
