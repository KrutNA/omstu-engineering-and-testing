struct Input {
    overall_count: u64,
    saw_left: u64,
    saw_right: u64,
}

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
	let numbers = value.split_whitespace()
	    .map(|v| v.parse::<u64>())
	    .collect::<Result<Vec<_>, _>>()
	    .map_err(|e| format!("Couldn't parse numbers: {}.", e))?;

	if numbers.len() != 3 {
	    Err(format!("Expected 3 numbers, got: {}.", numbers.len()))
		
	} else if numbers[0] > 13 {
	    Err(format!("Expected N not bigger then 13, but found {}.", numbers[0]))
	} else if numbers[1] >= numbers[0] || numbers[2] >= numbers[0] {
	    Err(format!("Exepected second and third numbers to be below first one."))
	} else {
	    Ok(Self {
		overall_count: numbers[0],
		saw_left: numbers[1],
		saw_right: numbers[2],
	    })
	}
    }
}

fn read_line() -> Result<String, String> {
    let mut line = String::new();

    std::io::stdin()
	.read_line(&mut line)
	.map_err(|e| e.to_string())?;

    line = line.trim().into();

    Ok(line)
}

fn solve(input: Input) -> u64 {
    SOLUTION[input.overall_count as usize]
	[input.saw_left as usize]
	[input.saw_right as usize]
}

const SIZE: usize = 14;
static SOLUTION: [[[u64; SIZE]; SIZE]; SIZE] = precalculate_solution();

const fn precalculate_solution() -> [[[u64; SIZE]; SIZE]; SIZE] {
    let mut buffer = [[[0u64; SIZE]; SIZE]; SIZE];

    buffer[1][1][1] = 1;
    let mut n = 2;
    while n != SIZE {
	let mut p = 1;
	while p <= n {
	    let mut r = 1;
	    while r <= n {
		buffer[n][p][r] = buffer[n - 1][p][r] * (n - 2) as u64
		    + buffer[n - 1][p - 1][r]
		    + buffer[n - 1][p][r - 1];
		r += 1;
	    }
	    p += 1;
	}
	n += 1;
    }
    
    buffer
}

fn main() {
    let blocks_count = match read_line().map(|v| v.parse::<u64>()) {
	Err(e) => { eprintln!("{}", e); return; },
	Ok(Err(e)) => { eprintln!("{}", e); return; },
	Ok(Ok(v)) => v
    };

    let lines = (1..=blocks_count)
	.map(|i| read_line()
	     .map(|v| v.as_str().try_into())
	     .map_err(|e| format!("Got an IO error on line {}: {}", i, e)))
	.collect::<Result<Vec<Result<Input, String>>, _>>();

    let lines = match lines {
	Err(e) => { eprintln!("{}", e); return; },
	Ok(v) => v,
    };

    for line in lines {
	match line {
	    Err(e) => eprintln!("{}", e),
	    Ok(v) => println!("{}", solve(v)),
	}
    }
}
