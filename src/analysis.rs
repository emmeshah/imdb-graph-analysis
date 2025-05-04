// Analysis
// Purpose: To analyze the movie graph through computing centrality metrics, clustering, and genre trends.

use petgraph::graph::NodeIndex;
use petgraph::visit::{Bfs};
use petgraph::algo::{dijkstra};
use petgraph::Graph;
use crate::graph_builder::MovieNode;
use std::collections::{HashMap, HashSet};
use petgraph::Undirected;

/// Computing degree centrality for all nodes in the graph.
pub fn degree_centrality(graph: &Graph<MovieNode, f64, Undirected>) -> HashMap<String, usize> {
    let mut centrality = HashMap::new();
    for node in graph.node_indices() {
        let movie = &graph[node];
        centrality.insert(movie.title.clone(), graph.edges(node).count());
    }
    centrality
}

/// Computing betweenness centrality using a simple approximation (based on shortest paths).
pub fn betweenness_centrality(graph: &Graph<MovieNode, f64, Undirected>) -> HashMap<String, f64> {
    let mut centrality = HashMap::new();
    let nodes: Vec<NodeIndex> = graph.node_indices().collect();
    for &src in &nodes {
        let paths = dijkstra(graph, src, None, |_| 1.0);
        for (&dst, &dist) in &paths {
            if src != dst && dist > 0.0 {
                let movie = &graph[dst];
                *centrality.entry(movie.title.clone()).or_insert(0.0) += 1.0;
            }
        }
    }
    centrality
}

/// Grouping movies by decade and returns average score gaps.
pub fn average_score_gap_by_decade(movies: &[crate::data_cleaning::CleanMovie]) -> HashMap<u16, f64> {
    let mut decade_map: HashMap<u16, Vec<f64>> = HashMap::new();
    for movie in movies {
        let decade = (movie.year / 10) * 10;
        decade_map.entry(decade).or_default().push(movie.score_gap);
    }
    decade_map
        .into_iter()
        .map(|(decade, gaps)| {
            let avg = gaps.iter().sum::<f64>() / gaps.len() as f64;
            (decade, avg)
        })
        .collect()
}

/// Finding the average critic-user score gap per genre.
pub fn average_score_gap_by_genre(movies: &[crate::data_cleaning::CleanMovie]) -> HashMap<String, f64> {
    let mut genre_map: HashMap<String, Vec<f64>> = HashMap::new();
    for movie in movies {
        for genre in &movie.genres {
            genre_map.entry(genre.clone()).or_default().push(movie.score_gap);
        }
    }
    genre_map
        .into_iter()
        .map(|(genre, gaps)| {
            let avg = gaps.iter().sum::<f64>() / gaps.len() as f64;
            (genre, avg)
        })
        .collect()
}

/// Clustering movies by connected components in the graph.
pub fn find_clusters(graph: &Graph<MovieNode, f64, Undirected>) -> Vec<Vec<String>> {
    let mut visited = HashSet::new();
    let mut clusters = Vec::new();

    for node in graph.node_indices() {
        if visited.contains(&node) {
            continue;
        }

        let mut cluster = Vec::new();
        let mut bfs = Bfs::new(graph, node);
        while let Some(nx) = bfs.next(graph) {
            if visited.insert(nx) {
                cluster.push(graph[nx].title.clone());
            }
        }
        clusters.push(cluster);
    }

    clusters
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_average_score_gap_by_decade() {
        use crate::data_cleaning::CleanMovie;
        let sample = vec![
            CleanMovie {
                title: "Test".into(),
                year: 1994,
                genres: vec!["Drama".into()],
                critic_score: 7.5,
                user_score: 8.2,
                score_gap: 0.7,
            },
            CleanMovie {
                title: "Test2".into(),
                year: 1999,
                genres: vec!["Drama".into()],
                critic_score: 6.5,
                user_score: 7.5,
                score_gap: 1.0,
            },
        ];
        let result = average_score_gap_by_decade(&sample);
        assert_eq!(result.get(&1990), Some(&0.85));
    }
} 
