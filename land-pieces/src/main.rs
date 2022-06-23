
fn solve(number: u64) -> u64 {
    match number {
	0 => 1,
	1 => 1,
	2 => 2,
	3 => 4,
	n => n*(n-1) / 2
	    + n*(n-1)*(n-2)*(n-3) / 24
	    + 1
    }
}

fn read_line() -> Result<String, String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
	Err(e) => Err(e.to_string()),
	_ => Ok(line),
    }
}

fn main() {
    let count = match read_line().map(|line| line.parse::<u64>()) {
	Ok(Ok(n)) => n,
	Ok(Err(e)) => {
	    eprint!("Couln't parse number of inputs: {}", e);
	    return;
	},
	Err(e) => {
	    eprintln!("Got an io error: {}", e);
	    return;
	},
    };

    let lines = (1..=count)
        .map(|_| read_line().map(|v| v.parse::<u64>()))
        .collect::<Result<Vec<_>, _>>();

    let lines = match lines {
	Err(e) => {
	    eprintln!("Got an io error: {}", e);
	    return;
	},
	Ok(lines) => lines,
    };

    for line in lines {
	match line {
	    Ok(number) => println!("{}", solve(number)),
	    Err(e) => eprintln!("{}", e)
	}
    }
}
