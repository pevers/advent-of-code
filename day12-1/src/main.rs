#![feature(exclusive_range_pattern)]

use std::collections::HashSet;
type Edge = (String, String);

fn main() {
    let mut nodes = HashSet::new();
    let edges: Vec<Edge> = include_str!("../input-test")
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
            // Is a connection
            route.iter().rev().next().unwrap().1 == edge.0
                // Is not already in our graph (looping)
                && route.iter().all(|c| c != edge)
                // Is not a small cave already visited
                && route.iter().filter(|node| {
                        node.0.as_bytes()[0] > b'Z'
                }).all(|n| n.0 != edge.0)
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
