// Graph Builder
// Purpose: To construct a graph where nodes are movies and edges represent genre similarity and score pattern similarity.

use petgraph::graph::{Graph};
use petgraph::Undirected;
use crate::data_cleaning::CleanMovie;

// A struct for representing a node in the graph.
#[derive(Debug, Clone)]
pub struct MovieNode {
    pub title: String,
    pub year: u16,
    pub genres: Vec<String>,
    pub critic_score: f64,
    pub user_score: f64,
}

// Function for cosine similarity between two vectors (critic_score, user_score).
fn cosine_similarity(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dot = a.0 * b.0 + a.1 * b.1;
    let norm_a = (a.0.powi(2) + a.1.powi(2)).sqrt();
    let norm_b = (b.0.powi(2) + b.1.powi(2)).sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

// Building an undirected graph where edges represent genre or score pattern similarity.
pub fn build_graph(movies: &[CleanMovie], genre_weight: f64, score_weight: f64, similarity_threshold: f64) -> Graph<MovieNode, f64, Undirected> {
    let mut graph = Graph::<MovieNode, f64, Undirected>::new_undirected();
    let mut indices = Vec::new();

    // Adding all movies as nodes
    for movie in movies {
        let node = MovieNode {
            title: movie.title.clone(),
            year: movie.year,
            genres: movie.genres.clone(),
            critic_score: movie.critic_score,
            user_score: movie.user_score,
        };
        indices.push(graph.add_node(node));
    }

    // Creating edges between similar movies
    for i in 0..movies.len() {
        for j in (i + 1)..movies.len() {
            let genre_overlap = genre_jaccard(&movies[i].genres, &movies[j].genres);
            let score_sim = cosine_similarity(
                (movies[i].critic_score, movies[i].user_score),
                (movies[j].critic_score, movies[j].user_score),
            );

            // Weighted sum of similarities
            let total_similarity = genre_weight * genre_overlap + score_weight * score_sim;

            if total_similarity >= similarity_threshold {
                graph.add_edge(indices[i], indices[j], total_similarity);
            }
        }
    }

    graph
}

/// Computing Jaccard similarity between two genre lists.
fn genre_jaccard(g1: &[String], g2: &[String]) -> f64 {
    let set1: std::collections::HashSet<_> = g1.iter().collect();
    let set2: std::collections::HashSet<_> = g2.iter().collect();
    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = (8.0, 7.0);
        let b = (7.0, 9.0);
        let sim = cosine_similarity(a, b);
        assert!(sim > 0.9);
    }

    #[test]
    fn test_genre_jaccard() {
        let g1 = vec!["Drama".into(), "Comedy".into()];
        let g2 = vec!["Drama".into(), "Action".into()];
        let sim = genre_jaccard(&g1, &g2);
        assert_eq!(sim, 1.0 / 3.0);
    }
} 
