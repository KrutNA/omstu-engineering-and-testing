
fn read_info() -> Option<(usize, usize)> {
    let mut line = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut line) {
	eprintln!("Got an IO error: {}", e);
	return None;
    }

    let numbers = line
	.trim()
	.split_whitespace()
	.map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>();

    match numbers {
	Ok(v) if v.len() == 2 => Some((v[0], v[1])),
	Ok(v) => {
	    eprintln!("Expected only 2 numbers, found: {}.", v.len());
	    None
	},
	Err(e) => {
	    eprintln!("{}", e);
	    None
	}
    }
}

fn read_number() -> Option<usize> {
    let mut line = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut line) {
	eprintln!("Got an IO error: {}", e);
	return None;
    }

    match line.trim().parse() {
	Ok(v) => Some(v),
	Err(e) => {
	    eprintln!("Couldn't parse number: {}", e);
	    return None
	}
    }
}

fn read_cross_info() -> Option<(usize, usize, usize)> {
    let mut line = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut line) {
	eprintln!("Got an IO error: {}", e);
	return None;
    }

    if line.trim().is_empty() {
	return None;
    }

    let numbers = line
	.trim()
	.split_whitespace()
	.map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>();

    match numbers {
	Ok(v) if v.len() == 3 => Some((v[0], v[1], v[2])),
	Ok(v) => {
	    eprintln!("Expected only 2 numbers, found: {}.", v.len());
	    None
	},
	Err(e) => {
	    eprintln!("{}", e);
	    None
	}
    }
}

fn process(
    idx: usize,
    paths: &mut Vec<Vec<[usize; 2]>>,
    distantions: &mut Vec<usize>,
    depos: &mut Vec<usize>,
    crosses: &mut Vec<usize>,
    crosses_count: usize,
) {
    let mut queue = vec![0; crosses_count];
    let mut used = [false; 500];
    distantions[depos[idx]] = 0;
    queue.push(depos[idx]);
    
    while let Some(current) = queue.pop() {
	used[current] = false;
	for i in 0..crosses[current] {
	    let new = distantions[current] + paths[current][i][1];
	    
	    if distantions[paths[current][i][0]] > new {
		distantions[paths[current][i][0]] = new;
		
		if !used[paths[current][i][0]] {
		    queue.push(paths[current][i][0]);
		    used[paths[current][i][0]] = true;
		}
	    }
	}
    }
}

fn main() {
    let blocks_count = match read_number() {
	Some(v) => v,
	None => return,
    };

    for block_idx in 0..blocks_count {
	let (depos_count, crosses_count) = match read_info() {
	    Some(v) => v,
	    None => return,
	};

	let mut depos = Vec::with_capacity(depos_count);

	for _ in 0..depos_count {
	    match read_number() {
		Some(v) => depos.push(v),
		None => return,
	    }
	}

	let mut crosses = vec![0; crosses_count];
	let mut distantions = vec![usize::MAX, crosses_count];

	let mut paths = vec![vec![[usize::MAX; 2]; crosses_count]; crosses_count];

	while let Some((from, to, length)) = read_cross_info() {
	    paths[from][crosses[from]][0] = to;
	    paths[from][crosses[from]][1] = length;
	    crosses[from] += 1;

	    paths[to][crosses[to]][0] = from;
	    paths[to][crosses[to]][1] = length;
	    crosses[to] += 1;
	}

	for depo_idx in 0..depos_count {
	    process(depo_idx, &mut paths, &mut distantions, &mut depos, &mut crosses, crosses_count);
	}

	let mut answer = usize::MAX;
	let mut anum = 0;

	for i in 1..=crosses_count {
	    process(i, &mut paths, &mut distantions, &mut depos, &mut crosses, crosses_count);
	    let mut max = usize::MIN;

	    for j in 1..crosses_count {
		if max < distantions[j] { max = distantions[j]; }
	    }

	    if answer > max { answer = max; anum = i; }
	}

	println!("{}", anum);
    }

    println!();
}
