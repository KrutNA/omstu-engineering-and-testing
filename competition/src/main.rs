use std::{convert::TryFrom, hash::Hash, collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum StateResult {
    Correct,
    Incorrect,
    ClarificationRequest,
    Unjudged,
    ErroneousSubmission,
}

impl TryFrom<&str> for StateResult {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" => Ok(StateResult::Correct),
            "I" => Ok(StateResult::Incorrect),
            "R" => Ok(StateResult::ClarificationRequest),
            "U" => Ok(StateResult::Unjudged),
            "E" => Ok(StateResult::ErroneousSubmission),
            _ => Err(format!("Unknown game result \"{}\".", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub participant: u64,
    pub task: u64,
    pub time: u64,
    pub result: StateResult,
}

impl TryFrom<&str> for State {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values = value.split_whitespace().collect::<Vec<_>>();

        if values.len() != 4 {
            return Err("Expected 4 arguments.".into());
        }

        Ok(Self {
            participant: values[0].parse().map_err(|_| "Expected participant id as number".to_string())?,
            task: values[1].parse().map_err(|_| "Expected task id as number".to_string())?,
            time: values[2].parse().map_err(|_| "Expected time as number".to_string())?,
            result: values[3].try_into()?,
        })
    }
}

const TIME_FOR_INCORRECT: u64 = 20;

#[derive(Debug, Eq, Clone)]
pub struct Total {
    pub participant: u64,
    pub solves: u64,
    pub time: u64,
}

impl Total {
    pub fn new(participant: u64) -> Self {
        Self {
            participant,
            solves: 0,
            time: 0,
        }
    }

    pub fn add_solve(&mut self) {
        self.solves += 1
    }

    pub fn add_time(&mut self, time: u64) {
        self.time += time;
    }
}

impl PartialEq for Total {
    fn eq(&self, other: &Self) -> bool {
        self.participant == other.participant
    }
} 

impl Ord for Total {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.solves.cmp(&other.solves)
            .then(self.time.cmp(&other.time).reverse())
            .then(self.participant.cmp(&other.participant))
    }
}

impl PartialOrd for Total {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Total {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.participant.hash(state)
    }
}

impl Display for Total {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.participant, self.solves, self.time))
    }
}

fn convert_io_error(err: std::io::Error) -> String {
    format!("Got an io error: \"{}\"", err)
}

fn solve_from_stdin() -> Result<Vec<Vec<Total>>, String> {
    let mut line = String::new();

    let count = {
        let _ = std::io::stdin()
            .read_line(&mut line)
            .map_err(convert_io_error)?;

        line.trim()
            .parse::<u64>()
            .map_err(|_| format!("Expected unsigned integer on first line, found: \"{}\".", line.trim()))?
    };

    line.clear();

    std::io::stdin()
        .read_line(&mut line)
        .map(|_| match line.trim() {
            clean if clean.is_empty() => {
		line.clear();
		Ok(())
	    },
            _ => Err("Expected empty line after first.".to_string())
        })
        .map_err(convert_io_error)??;

    (1..=count)
        .map(|i| {
            let mut totals: HashSet<Total> = HashSet::new();
            
            (1..)
                .map_while(|j| {
                    let mut line = String::new();

                    let res = std::io::stdin()
                        .read_line(&mut line)
                        .map_err(|e| e.to_string());

                    match (res, line.trim()) {
                        (Err(e), _) => Some(Err(e)),
                        (_, line) if line.is_empty() => None,
                        (_, line) => Some(line.try_into()),
                    }
                    .map(|v: Result<State, _>|
			 v.map_err(|e| format!("line {}: {}", j, e)))
                })
                .try_for_each(|v| v.map(|state| {
                    let default = Total::new(state.participant);
                    let total = &mut totals.take(&default).unwrap_or(default);

		    match state.result {
                        StateResult::Correct => {
                            total.add_solve();
                            total.add_time(state.time);
                        },
                        StateResult::Incorrect => total.add_time(TIME_FOR_INCORRECT),
                        _ => (),
                        
                    }
                    totals.insert(total.to_owned());
                }))
                .map_err(|e| format!("Got and error in block {}: {}", i, e))?;

            let mut res = totals.into_iter().collect::<Vec<_>>();
            res.sort_unstable_by(|l, r| l.cmp(r).reverse());
            Ok(res)
            
        })
        .collect::<Result<_, _>>()
}

fn main() {
    match solve_from_stdin() {
        Ok(totals) => totals
            .iter()
            .for_each(|v| {
                v.iter().for_each(|v| println!("{}", v));
                println!()
            }),
        Err(e) => eprintln!("{}", e),
    }
}


#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::Total;

    #[test]
    fn test_total_compare() {
        let a = Total {
            participant: 1,
            solves: 5,
            time: 666,
        };
        let b = Total {
            participant: 2,
            solves: 3,
            time: 1,
        };

        assert_eq!(a.cmp(&b), Ordering::Greater);

        let b = Total {
            participant: 2,
            solves: 5,
            time: 1000,
        };

        assert_eq!(a.cmp(&b), Ordering::Greater);

        assert_eq!(a.cmp(&a), Ordering::Equal);
    }
}
