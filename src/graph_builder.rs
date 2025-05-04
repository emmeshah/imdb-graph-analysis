// Graph Builder
// Purpose: I want to construct a graph where nodes are movies and edges
// represent genre similarity and score pattern similarity.

use petgraph::graph::{Graph};
use petgraph::Undirected;
use crate::data_cleaning::CleanMovie;

// A struct for representing a node in the graph.
// This contains only the key info needed for the camparison and labeling
// Fields:
// title: the title of the movie as a string
// year: the year of release
// genres: listing of the genre tags
// critic_score: the normalized metacritic score on that scale of 0 to 10
// user score: IMBDb score from 0 to 10

#[derive(Debug, Clone)]
pub struct MovieNode {
    pub title: String,
    pub year: u16,
    pub genres: Vec<String>,
    pub critic_score: f64,
    pub user_score: f64,
}

// Function for calculating cosine similarity between two vectors (critic_score, user_score).
// The arguments are a and b which are tuples that represent critic score, user_score
// This function will return the similarity score which is between 0 and 1
// To do so, it will compute the dot product and divide it by the product of the vector norms
// It will return a 0 if either vector is zero length but if not, it will continue as expected

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

// This function constructs an undirected graph where the node is a movie and
// each node will represent a combined similarity based on the genre overlap
// and the score similarities.
// The arguments are the movies which is taken from cleaned movie data,
// genre_weight which is the weight for the jaccard similarity of genre,
// score_weight which is the cosine score similarity,
// similarity_threshold which is the mimunum required to form an edge
// All together, the function returns a graph of the movie nodes with weighted edges
// To do so, it will add the movies as node, then compute the similarity for unique pairs and
// add the edge if the weighted sum will be higher than the minimum (the threshold)
pub fn build_graph(movies: &[CleanMovie], genre_weight: f64, score_weight: f64, similarity_threshold: f64) -> Graph<MovieNode, f64, Undirected> {
    let mut graph = Graph::<MovieNode, f64, Undirected>::new_undirected();
    let mut indices = Vec::new();

    // adding all movies as nodes
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

    // creating edges between similar movies through iterating over the unique pairs
    for i in 0..movies.len() {
        for j in (i + 1)..movies.len() {
            let genre_overlap = genre_jaccard(&movies[i].genres, &movies[j].genres); //this is the jaccard similarlity between the genre sets
            let score_sim = cosine_similarity(
                (movies[i].critic_score, movies[i].user_score),
                (movies[j].critic_score, movies[j].user_score),
            );

            // weighted sum of similarities
            let total_similarity = genre_weight * genre_overlap + score_weight * score_sim;

            // adding edge if similarity meets or exceeds the threshold
            if total_similarity >= similarity_threshold {
                graph.add_edge(indices[i], indices[j], total_similarity);
            }
        }
    }

    graph
}

// Computing Jaccard similarity between two genre vectors.
// Arguments will be g1 and g2 which are from the genre strings
// This will return f64 between 0 and 1 to represent the overlap
// To do so, the intersection will be divided by the union

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

// Unit testing to verify the correctness of the similarity function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = (8.0, 7.0);
        let b = (7.0, 9.0);
        let sim = cosine_similarity(a, b);
        assert!(sim > 0.9); // expecting strong similarity between the vectors that have close ratios
    }

    #[test]
    fn test_genre_jaccard() {
        let g1 = vec!["Drama".into(), "Comedy".into()];
        let g2 = vec!["Drama".into(), "Action".into()];
        let sim = genre_jaccard(&g1, &g2);
        assert_eq!(sim, 1.0 / 3.0); // wanting one shared genre out of the three total
    }
} 
