use std::collections::{HashMap, HashSet};
type Edge = (String, String);

// This is slow as d*cks but it works
// This is a naiive implementation that does a BFS
// I should look into a DFS Dynamic Programming solution
// to speed this up massively. I just don't have the brainpower anymore :)

fn main() {
    let mut nodes = HashSet::new();
    let edges: Vec<Edge> = include_str!("../input")
        .split("\n")
        .flat_map(|l| {
            let mut routes = l.split("-");
            let (from, to) = (routes.next().unwrap(), routes.next().unwrap());
            nodes.insert(from.to_string());
            nodes.insert(to.to_string());
            [
                (to.to_string(), from.to_string()),
                (from.to_string(), to.to_string()),
            ]
        })
        .collect();
    let routes: Vec<Vec<Edge>> = edges
        .iter()
        .filter(|edge| edge.0 == "start")
        .flat_map(|edge| find_routes(&mut vec![edge.clone()], &edges))
        .collect();
    println!("{:?}", routes.len());
}

fn find_routes(route: &mut Vec<Edge>, edges: &Vec<Edge>) -> Vec<Vec<Edge>> {
    if route.iter().rev().next().unwrap().1 == "end" {
        return vec![route.clone()].to_vec();
    }
    let connections: Vec<&Edge> = edges
        .iter()
        .filter(|&edge| {
            // Is connected and not the start node
            let connection = &route.iter().rev().next().unwrap().1;
            if *connection != edge.0 || edge.0 == "start" {
                return false;
            }
            // At most 2 small cave visits 
            let mut small_cave_visits: HashMap<String, u8> = HashMap::new();
            route.iter().for_each(|node| {
                if node.0.as_bytes()[0] > b'Z' {
                    *small_cave_visits.entry(node.0.clone()).or_insert(0) += 1;
                }
            });
            *small_cave_visits.entry(edge.0.clone()).or_insert(0) += 1;
            return small_cave_visits.values().filter(|&&v| v > 1).count() <= 1
                && small_cave_visits.values().all(|&v| v <= 2);
        })
        .collect();
    let mut new_routes = Vec::new();
    connections.iter().for_each(|&c| {
        let mut new_route = route.clone();
        new_route.push(c.clone());
        new_routes.append(&mut find_routes(&mut new_route, edges));
    });
    new_routes
}
