use std::io::Read;

#[derive(PartialEq, Eq, Debug)]
struct Input {
    index: usize,
    weight: u64,
    iq: u64,
}

impl TryFrom<(usize, &str)> for Input {
    type Error = String;

    fn try_from(value: (usize, &str)) -> Result<Self, Self::Error> {
	let numbers = value.1.split_whitespace()
	    .map(|v| v.parse::<u64>())
	    .collect::<Result<Vec<_>, _>>()
	    .map_err(|e| format!("Coldn't parse numbers on line {}: {}.",
				 value.0, e))?;
	if numbers.len() != 2 {
	    return Err(format!("Expected 2 numbers on line {}, found {}.",
			       value.0, numbers.len()));
	}

	Ok(Self {
	    index: value.0,
	    weight: numbers[0],
	    iq: numbers[1],
	})
    }
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
	self.weight.partial_cmp(&other.weight).and(other.iq.partial_cmp(&self.iq))
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	self.partial_cmp(&other).unwrap()
    }
}



fn main() {
    let mut data = String::new();

    // <Is this a GoLang now?>.jpg
    if let Err(e) = std::io::stdin().read_to_string(&mut data) {
	eprintln!("Got an IO error: {}", e);
	return;
    }

    let inputs = data
	.split_terminator("\n")
        .enumerate()
        .map(|(idx, v)| (idx, v.trim()).try_into())
        .collect::<Result<Vec<Input>, _>>();

    let mut inputs = match inputs {
	Ok(v) => v,
	Err(e) => {
	    eprintln!("{}", e);
	    return;
	},
    };

    inputs.sort_unstable();

    let mut longest_idxs = vec![];

    for idx in 0..(inputs.len() - 1 - longest_idxs.len()) {
	let mut idxs = Vec::with_capacity(inputs.len());
	idxs.push(idx);
	let mut prev = &inputs[idx];

	for i in idx + 1..inputs.len() - 1 {
	    let cur = &inputs[i];
	    if prev.weight < cur.weight && prev.iq > cur.iq {
		idxs.push(i);
		prev = cur;
	    }
	}

	if idxs.len() > longest_idxs.len() {
	    longest_idxs = idxs;
	}
    }

    println!("{}", longest_idxs.len());
    longest_idxs.iter()
	.for_each(|&idx| println!("{}", inputs[idx].index + 1));
}
