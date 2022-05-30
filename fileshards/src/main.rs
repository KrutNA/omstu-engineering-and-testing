use std::{cmp::Ordering, ops::Add, collections::{VecDeque, HashSet}};

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

fn read_data_block(line_idx: i32) -> Option<Result<String, String>> {
    let mut line = String::new();

    std::io::stdin()
	.read_line(&mut line)
        .map_err(|e| e.to_string())
        .and(match line.trim() {
	    line if line.is_empty() => Ok(None),
	    line if line.chars().all(|v| v == '0' || v == '1') => Ok(Some(line.into())),
	    _ => Err("Found unknown symbols in file.".into()),
	})
        .map_err(|e| format!("line {}: {}", line_idx, e))
        .transpose()
}

type Part = (Option<String>, Option<String>);
type Parts = Vec<Part>;

fn solve_block_line(part: String, parts: &mut Parts) {
    let (index, half) = (part.len() % 4, (part.len() % 8) / 4);

    match &mut parts[index] {
	parts if index == 0 => match parts {
	    (None, _) => parts.0 = Some(part),
	    (Some(high), None) => match high.len().cmp(&part.len()) {
		Ordering::Greater => parts.1 = Some(part),
		Ordering::Less => {
		    let high = parts.0.replace(part).unwrap();
		    parts.1 = Some(high);
		},
		Ordering::Equal if *high != part => parts.1 = Some(part),
		Ordering::Equal => (),
	    },
	    (Some(high), Some(low)) => match (high.len().cmp(&part.len()),
					      low.len().cmp(&part.len())) {
		(Ordering::Greater, _) => parts.0 = Some(part),
		(_, Ordering::Less) => parts.1 = Some(part),
		_ => (),
	    },
	},

	(None, _) if half == 0 => parts[index].0 = Some(part),

	(_, None) if half == 1 => parts[4 - index].1 = Some(part),
	

	(Some(high), _) if half == 0 && high.len() < part.len()
	    => parts[index].0 = Some(part),

	(_, Some(low)) if half == 1 && low.len() > part.len()
	    => parts[4 - index].1 = Some(part),

	_ => (),
    }
}

fn create_combinations(parts: &Part) -> Vec<String> {
    match parts {
	(None, None) => vec![],
	(Some(v), None) => vec![v.to_string().add(v)],
	(Some(v1), Some(v2)) => vec![v1.to_string().add(v2),
				     v2.to_string().add(v1)],
	_ => unreachable!()
    }
}

fn solve_from_stdio_block_smart_unstable(block_idx: usize) -> Result<String, String> {
    // First string is the longest part, seconds is the shortest.
    let mut parts = vec![
	(None, None), (None, None), (None, None), (None, None), (None, None)
    ];

    (1..)
	.map_while(read_data_block)
	.try_for_each(|v| v.map(|part| solve_block_line(part, &mut parts)))
	.map_err(|e| format!("Got an error in block {}: {}", block_idx, e))?;
    
    print!("{:?}", parts);

    let mut combinations = parts.iter()
	.flat_map(create_combinations)
	.collect::<VecDeque<_>>();

    print!("{:?}", combinations);

    while let Some(elt) = combinations.pop_back() {
	if combinations.contains(&elt) {
	    return Ok(elt);
	}
    }

    Err(format!("Couldn't solve block {}.", block_idx))
}

fn satisfies_remove_requirements(part: &str, elt: &str) -> bool {
    part.len() % 8 == elt.len() % 8
	&& part.len() != elt.len()
	&& (part.len() % 8 / 4 == 0 && part.len() > elt.len()
	    || part.len() % 8 / 4 == 1 && part.len() < elt.len())
}

fn remove_all_matches(parts: &mut Vec<String>) {
    for i in 0..parts.len() {

	let mut j = i + 1;
	while j != parts.len() {
	    if satisfies_remove_requirements(&parts[i], &parts[j]) {
		parts.remove(j);
	    } else {
		j += 1;
	    }
	}
    }
}

fn solve_from_stdio_block_stable_inner() -> Result<Result<String, String>, String> {
    let mut parts = &mut (1..)
	.map_while(read_data_block)
	.collect::<Result<HashSet<_>, _>>()?
	.into_iter()
	.collect::<Vec<_>>();

    remove_all_matches(&mut parts);

    let combinations = parts
	.iter()
        .flat_map(|part| parts.iter()
		  .filter(|elt| (part.len() + elt.len()) % 8 == 0)
		  .map(|elt| format!("{}{}", elt, part))
		  .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut uniques = combinations
        .iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|&v| (v, combinations.iter().filter(|&c| v.eq(c)).count()))
        .collect::<Vec<_>>();

    uniques.sort_unstable_by(|l, r| l.1.cmp(&r.1));

    Ok(uniques.last()
       .map(|v| v.0.clone())
       .ok_or("Couldn't solve.".into()))
}

fn solve_from_stdio_block_stable(block_idx: usize) -> Result<Result<String, String>, String> {
    solve_from_stdio_block_stable_inner()
	.map_err(|e| format!("Error on block {}: {}", block_idx, e))
}

fn solve_from_stdin() -> Result<Vec<Result<String, String>>, String> {
    let count = read_count()?;
    read_first_empty_line()?;

    (1..=count)
	.map(solve_from_stdio_block_stable)
	.collect()
}

fn main() {
    match solve_from_stdin() {
	Ok(v) => v
	    .iter()
	    .for_each(|v| match v {
		Ok(v) => println!("{}", v),
		Err(e) => eprintln!("{}", e),
	    }),
	Err(e) => eprintln!("{}", e),
    }
}
