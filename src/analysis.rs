// Analysis
// Purpose: I want to analyze the movie graph through computing
// centrality metrics, clustering, and genre trends.

use petgraph::graph::NodeIndex;
use petgraph::visit::{Bfs};
use petgraph::algo::{dijkstra};
use petgraph::Graph;
use crate::graph_builder::MovieNode;
use std::collections::{HashMap, HashSet};
use petgraph::Undirected;

// This will compute degree centrality for all nodes in the graph.
// The argument is a graph for an undirected graph of the MovieNodes
// This will return a HashMap that has they keys being teh movie title
// and the value is the number of connections or the degrees
// To do so, it will iterate over all edges
// and then count how many edges it has
pub fn degree_centrality(graph: &Graph<MovieNode, f64, Undirected>) -> HashMap<String, usize> {
    let mut centrality = HashMap::new();
    for node in graph.node_indices() {
        let movie = &graph[node];
        centrality.insert(movie.title.clone(), graph.edges(node).count());
    }
    centrality
}

// Computing betweenness centrality by counting the amount
// of times a node appears in the shortest paths
// The argument is an undirected graph
// This will return a HashMap of the keys being the movie title
// and the value is a float that represents centrality
// To do this, for every node that has a source, it will run Dijstra's algorithm
// Then it will count how often each destination node appears in paths from others
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

// Grouping movies by decade and then computing the average score gap for each
// The arguments here is just the movies from the Clean Movie struct
// This will result in the returning of a HashMap that will have the decade to average score_gap
// To do so, it will group the score gaps by decade by normalizing the year to the decade
// And then computing the average gap per decade in each group
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

// Finding the average critic-user score gap per genre.
// The argument is again the movies from the Clean Movie struct
// Similarly to the last, it will return a HashMap but this time will have genre name to average score_gap
// To do so, each genre will have a score_gap value across the movies
// And then it will give the average gap per genre
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

// Clustering movies by connected components in the graph.
// The argument is a undirected graph
// This will return a vector of clusters that will be a list of movie titles
// To do so, it will use Breadh-First Search in order to explore the nodes
// Start a new cluster for each unvisited node
// THen the reachable nodes are added to the cluster
pub fn find_clusters(graph: &Graph<MovieNode, f64, Undirected>) -> Vec<Vec<String>> {
    let mut visited = HashSet::new(); // tracking the nodes already seen
    let mut clusters = Vec::new(); // final list of the clusterings

    for node in graph.node_indices() {
        if visited.contains(&node) {
            continue; // going to skip the visited nodes
        }

        let mut cluster = Vec::new();
        let mut bfs = Bfs::new(graph, node);
        while let Some(nx) = bfs.next(graph) { // exploring the connected components
            if visited.insert(nx) {
                cluster.push(graph[nx].title.clone());
            }
        }
        clusters.push(cluster); // adding the component to the result
    }

    clusters
}

//Testing to validate the average score gap by the decade
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
        let result = average_score_gap_by_decade(&sample); //expecting average gap for 1990s to be (0.7 + 1.0) / 2 = 0.85
        assert_eq!(result.get(&1990), Some(&0.85));
    }
} 
