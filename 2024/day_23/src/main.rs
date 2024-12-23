use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Connections = HashSet<String>;
type Graph = HashMap<String, Connections>;

fn parse_file(filename: &str) -> Graph {
    let mut result = Graph::new();

    for (left, right) in read_to_string(filename)
        .expect("Failed to open file")
        .lines()
        .map(|x| x.trim().split_once("-").expect("Invalid format in file"))
    {
        result
            .entry(left.to_string())
            .or_default()
            .insert(right.to_string());
        result
            .entry(right.to_string())
            .or_default()
            .insert(left.to_string());
    }

    result
}

fn find_size_3_components(graph: &Graph) -> HashSet<(String, String, String)> {
    let mut results = HashSet::new();
    let id_idx_map: HashMap<&str, u32> = graph
        .keys()
        .enumerate()
        .map(|(idx, id)| (id.as_str(), idx as u32))
        .collect();

    for (node_1, connections_1) in graph.iter() {
        let idx_1 = id_idx_map.get(node_1.as_str()).unwrap();
        for node_2 in connections_1.iter() {
            let idx_2 = id_idx_map.get(node_2.as_str()).unwrap();
            if idx_2 < idx_1 {
                continue;
            }
            for node_3 in connections_1
                .intersection(graph.get(node_2).unwrap())
                .into_iter()
            {
                let idx_3 = id_idx_map.get(node_3.as_str()).unwrap();
                if idx_3 < idx_2 {
                    continue;
                }
                results.insert((node_1.clone(), node_2.clone(), node_3.clone()));
            }
        }
    }

    results
}

fn num_size_3_components_with_t(graph: &Graph) -> u32 {
    let size_3_components = find_size_3_components(graph);

    size_3_components
        .iter()
        .filter(|(x, y, z)| x.starts_with("t") || y.starts_with("t") || z.starts_with("t"))
        .count() as u32
}

fn bron_kerbosch(
    current_clique: &mut HashSet<String>,
    candidates: &mut HashSet<String>,
    excluded: &mut HashSet<String>,
    graph: &Graph,
    largest_clique: &mut HashSet<String>,
) {
    if candidates.is_empty() && excluded.is_empty() {
        if current_clique.len() > largest_clique.len() {
            *largest_clique = current_clique.clone();
        }
        return;
    }

    for node in candidates.clone() {
        let neighbors = graph.get(&node).unwrap();

        current_clique.insert(node.clone());
        let mut new_candidates: HashSet<_> = candidates.intersection(&neighbors).cloned().collect();
        let mut new_excluded: HashSet<_> = excluded.intersection(&neighbors).cloned().collect();

        bron_kerbosch(
            current_clique,
            &mut new_candidates,
            &mut new_excluded,
            graph,
            largest_clique,
        );

        current_clique.remove(&node);
        candidates.remove(&node);
        excluded.insert(node);
    }
}

fn find_largest_clique(graph: &Graph) -> HashSet<String> {
    let mut largest_clique = HashSet::new();
    let mut all_nodes = graph.keys().cloned().collect();
    bron_kerbosch(
        &mut HashSet::new(),
        &mut all_nodes,
        &mut HashSet::new(),
        &graph,
        &mut largest_clique,
    );

    largest_clique
}

fn find_password(graph: &Graph) -> String {
    let mut sorted_nodes: Vec<_> = find_largest_clique(graph).iter().cloned().collect();
    sorted_nodes.sort();
    sorted_nodes.join(",")
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        find_largest_clique, find_password, find_size_3_components, num_size_3_components_with_t,
        parse_file,
    };

    #[test]
    fn test_size_3_components() {
        let graph = parse_file("testinput.txt");
        assert_eq!(find_size_3_components(&graph).len(), 12);
    }

    #[test]
    fn test_size_3_components_with_t() {
        let graph = parse_file("testinput.txt");
        assert_eq!(num_size_3_components_with_t(&graph), 7);
    }

    #[test]
    fn test_largest_clique() {
        let graph = parse_file("testinput.txt");
        let target_set: HashSet<String> = vec!["co", "de", "ka", "ta"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(find_largest_clique(&graph), target_set);
    }

    #[test]
    fn test_password() {
        let graph = parse_file("testinput.txt");
        assert_eq!(find_password(&graph), "co,de,ka,ta");
    }
}

fn main() {
    let graph = parse_file("input.txt");
    println!("Challenge 1: {}", num_size_3_components_with_t(&graph));
    println!("Challenge 2: {}", find_password(&graph));
}
