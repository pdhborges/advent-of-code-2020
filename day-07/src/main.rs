use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn reachable(source: &str, graph: &HashMap<&str, HashSet<&str>>) -> usize {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut pending: VecDeque<&str> = VecDeque::new();

    pending.push_back(source);

    while !pending.is_empty() {
        let current = pending.pop_front().unwrap();
        visited.insert(current);

        if graph.contains_key(current) {
            for next in &graph[current] {
                if !visited.contains(next) {
                    pending.push_back(next);
                }
            }
        }
    }

    return visited.len() - 1;
}

fn count_bags_inside<'a>(
    bag: &'a str,
    graph: &HashMap<&str, HashSet<&'a str>>,
    edge_weight: &HashMap<(&'a str, &'a str), usize>,
) -> usize {
    if graph.contains_key(bag) {
        let mut bags_inside: usize = 0;
        for bag_inside in &graph[bag] {
            let bag_inside_count = edge_weight[&(bag, *bag_inside)];
            bags_inside += bag_inside_count
                + bag_inside_count * count_bags_inside(bag_inside, graph, edge_weight);
        }
        return bags_inside;
    } else {
        return 0;
    }
}

fn main() -> Result<()> {
    let bag_extractor = Regex::new(r"(\d+) (.+)").unwrap();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input
        .replace(".", "")
        .replace(" bags", "")
        .replace(" bag", "");

    let mut contains: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut contained: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut contains_count: HashMap<(&str, &str), usize> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(" contain ");
        let container_bag = parts.next().unwrap();
        let contained_bags = parts.next().unwrap();

        for contained_bag in contained_bags.split(", ") {
            if contained_bag == "no other" {
                continue;
            }

            let cap = bag_extractor.captures(contained_bag).unwrap();
            let count: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let bag = cap.get(2).unwrap().as_str();

            contains_count.insert((container_bag, bag), count);

            let bags = contains.entry(container_bag).or_insert(HashSet::new());
            bags.insert(bag);

            let bags = contained.entry(bag).or_insert(HashSet::new());
            bags.insert(container_bag);
        }
    }

    writeln!(
        io::stdout(),
        "Answer 1: {}",
        reachable("shiny gold", &contained)
    )?;

    writeln!(
        io::stdout(),
        "Answer 2: {}",
        count_bags_inside("shiny gold", &contains, &contains_count)
    )?;

    return Ok(());
}
