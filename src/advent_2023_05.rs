use crate::helpers::{read_lines, IteratorExt};

#[allow(dead_code)]
fn run_a(filename: &str) -> usize {
    let parts = read_lines(filename).into_iter().split(&"".to_string());

    let seeds = parts[0][0]
        .split(" ")
        .into_iter()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let chunks = parts
        .into_iter()
        .skip(1)
        .map(parse_chunk)
        .collect::<Vec<Chunk>>();

    seeds
        .into_iter()
        .map(|x| recurse_a(&chunks, "seed", x))
        .min()
        .unwrap()
}

fn recurse_a(chunks: &Vec<Chunk>, from: &str, from_value: usize) -> usize {
    if from == "location" {
        return from_value;
    }

    let chunk = chunks.iter().find(|x| &x.from == from).unwrap();

    let target_option = chunk
        .maps
        .clone()
        .into_iter()
        .filter(|x| x.source_start < from_value && x.source_end >= from_value)
        .first_option();

    let result = match target_option {
        Some(target) => target.target_start + (from_value - target.source_start),
        None => from_value,
    };

    recurse_a(chunks, &chunk.to, result)
}

#[allow(dead_code)]
pub fn run_b(filename: &str) -> usize {
    0
}

fn parse_chunk(lines: Vec<String>) -> Chunk {
    let (first, rest_lines) = lines.split_first().unwrap();

    let n1 = first.find('-').unwrap();
    let n2 = first.rfind(' ').unwrap();

    let from = &first[..n1];
    let to = &first[(n1 + 4)..n2];

    let maps: Vec<ItemMap> = rest_lines.into_iter().map(parse_map).collect();

    Chunk {
        from: from.to_string(),
        to: to.to_string(),
        maps,
    }
}

fn parse_map(a: &String) -> ItemMap {
    let splits = a
        .split(' ')
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    ItemMap {
        target_start: splits[0],
        source_start: splits[1],
        source_end: splits[1] + splits[2] - 1,
    }
}

#[derive(Debug, Clone)]
struct Chunk {
    from: String,
    to: String,
    maps: Vec<ItemMap>,
}

#[derive(Debug, Clone)]
struct ItemMap {
    target_start: usize,
    source_start: usize,
    source_end: usize,
}

#[cfg(test)]
mod test {
    use super::{run_a, run_b};
    use crate::test_base::test::run_test;

    #[test]
    fn a_sample() {
        run_test(file!(), "sample", run_a, 35);
    }

    #[test]
    fn a_input() {
        run_test(file!(), "input", run_a, 57075758);
    }

    #[test]
    fn b_sample() {
        run_test(file!(), "sample", run_b, 46);
    }

    #[test]
    fn b_input() {
        run_test(file!(), "input", run_b, 31161857);
    }
}
