use std::iter::once;

use crate::helpers::{intersect, is_intersect, read_lines, IteratorExt, IteratorExtClone};

#[allow(dead_code)]
fn run_a(filename: &str) -> isize {
    let parts = read_lines(filename).into_iter().split(&"".to_string());

    let seeds = parse_seeds(&parts[0][0]);

    let chunks = parse_chunks(parts);

    seeds
        .into_iter()
        .map(|x| recurse_a(&chunks, "seed", x))
        .min()
        .unwrap()
}

fn recurse_a(chunks: &Vec<Chunk>, from: &str, from_value: isize) -> isize {
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
fn run_b(filename: &str) -> isize {
    let parts = read_lines(filename).into_iter().split(&"".to_string());

    let seeds_pre = parse_seeds(&parts[0][0]);

    let seeds: Vec<_> = seeds_pre
        .into_iter()
        .pairs()
        .map(|(x, y)| (x, x + y - 1))
        .collect();

    let chunks = parse_chunks(parts);

    seeds
        .into_iter()
        .map(|(x, y)| recurse_b(&chunks, "seed", x, y))
        .min()
        .unwrap()
}

fn recurse_b(chunks: &Vec<Chunk>, from: &str, from_start: isize, from_end: isize) -> isize {
    if from == "location" {
        return from_start;
    }

    let chunk = chunks.iter().find(|x| &x.from == from).unwrap();

    let points: Vec<_> = chunk
        .maps
        .iter()
        .map(|a| a.source_start)
        .chain(chunk.maps.iter().map(|a| a.source_end))
        .distinct()
        .order()
        .collect();

    let parts = to_parts(from_start, from_end, points);

    parts
        .into_iter()
        .map(|(x, y)| calculate_b(chunks, chunk, x, y))
        .min()
        .unwrap()
}

fn calculate_b(chunks: &Vec<Chunk>, chunk: &Chunk, part_start: isize, part_end: isize) -> isize {
    let target_option = chunk
        .maps
        .iter()
        .filter(|x| is_intersect(x.source_start, x.source_end, part_start, part_end))
        .first_option();

    let (target_start, target_end) = if let Some(target) = target_option {
        let (start, end) =
            intersect(target.source_start, target.source_end, part_start, part_end).unwrap();

        let s = target.target_start - target.source_start;

        (start + s, end + s)
    } else {
        (part_start, part_end)
    };

    recurse_b(chunks, &chunk.to, target_start, target_end)
}

fn parse_seeds(line: &str) -> Vec<isize> {
    line
        .split(" ")
        .into_iter()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_chunks(parts: Vec<Vec<String>>) -> Vec<Chunk> {
    parts.into_iter().skip(1).map(parse_chunk).collect()
}

fn to_parts(from_start: isize, from_end: isize, points: Vec<isize>) -> Vec<(isize, isize)> {
    if from_start == from_end {
        return vec![(from_start, from_end)];
    }

    let middle = points
        .into_iter()
        .filter(|x| &from_start <= x && &from_end >= x);

    let temp = once(from_start).chain(middle).chain(once(from_end));

    temp.pairs_every().collect()
}

fn parse_chunk(lines: Vec<String>) -> Chunk {
    let (first, rest_lines) = lines.split_first().unwrap();

    let n1 = first.find('-').unwrap();
    let n2 = first.rfind(' ').unwrap();

    let from = &first[..n1];
    let to = &first[(n1 + 4)..n2];

    let maps: Vec<_> = rest_lines.into_iter().map(parse_map).collect();

    Chunk {
        from: from.to_string(),
        to: to.to_string(),
        maps,
    }
}

fn parse_map(a: &String) -> ItemMap {
    let splits: Vec<_> = a
        .split(' ')
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

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
    target_start: isize,
    source_start: isize,
    source_end: isize,
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
