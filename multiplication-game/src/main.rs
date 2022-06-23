enum GameWinner {
    Ollie,
    Stan,
}

fn solve(number: u64) -> GameWinner {
    const EPSILON: f64 = 1e-6;
    let lg = (number as f64).ln() / 18.0f64.ln();
    let lo = lg.floor();
    if lg - lo < EPSILON {
	GameWinner::Ollie
    } else if number as f64 / 18.0f64.powf(lo) > 9.0 {
	GameWinner::Ollie
    } else {
	GameWinner::Stan
    }
}

fn read_lines() -> Result<Vec<String>, String> {
    let mut lines = vec![];

    for line_idx in 1.. {
	let mut line = String::new();

	std::io::stdin()
	    .read_line(&mut line)
	    .map_err(|e| format!("IO error on line {}: {}", line_idx, e))?;

	line = line.trim().into();
	if line.len() == 0 { break; }
	lines.push(line);
    }

    Ok(lines)
}

fn main() {
    let lines = read_lines();
    match lines {
	Ok(lines) => lines
	    .iter()
	    .map(|v| v.parse::<u64>().map(solve))
	    .for_each(|v| match v {
		Ok(GameWinner::Ollie) => println!("Ollie wins"),
		Ok(GameWinner::Stan) => println!("Stan wins"),
		Err(e) => eprintln!("{}", e),
	    }),
	Err(e) => eprintln!("{}", e),
    }
}
