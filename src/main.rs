// Main
// Purpose: This is the main entry point.
// I want to load data, build graph, peroform analysis, and write output files.

mod data_cleaning;
mod graph_builder;
mod analysis;

use std::error::Error;
use std::env;
use std::fs::File;
use std::io::Write;
use data_cleaning::load_and_clean_data;
use graph_builder::build_graph;
use analysis::*;

fn main() -> Result<(), Box<dyn Error>> {
    // I want to parse the args: [binary] [csv_path] [genre_weight] [score_weight] [similarity_threshold]
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <csv_path> <genre_weight> <score_weight> <similarity_threshold>", args[0]);
        std::process::exit(1);
    }
    // Extracting arguments and converting types
    let csv_path = &args[1];
    let genre_weight: f64 = args[2].parse()?;
    let score_weight: f64 = args[3].parse()?;
    let threshold: f64 = args[4].parse()?;

    println!("Loading and cleaning data...");
    let movies = load_and_clean_data(csv_path)?;
    println!("{} movies loaded.", movies.len());

    println!("Building graph...");
    let graph = build_graph(&movies, genre_weight, score_weight, threshold);
    println!("Graph built with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    println!("Running analysis...");

    // Centrality metrics
    let degree = degree_centrality(&graph);
    let between = betweenness_centrality(&graph);

    // score gap trends
    let by_decade = average_score_gap_by_decade(&movies);
    let by_decade_str: std::collections::HashMap<String, f64> = 
        by_decade.iter().map(|(k, v)| (k.to_string(), *v)).collect();
    
    let by_genre = average_score_gap_by_genre(&movies);

    //finding clusters of related films in the graph
    let clusters = find_clusters(&graph);

    println!("Writing output to 'output/' directory...");
    std::fs::create_dir_all("output")?;
    
    //saving analysis results to the csv and txt files
    write_csv("output/degree_centrality.csv", &degree)?;
    write_csv("output/betweenness_centrality.csv", &between)?;
    write_csv("output/score_gap_by_decade.csv", &by_decade_str)?;
    write_csv("output/score_gap_by_genre.csv", &by_genre)?;
    write_clusters("output/movie_clusters.txt", &clusters)?;

    println!("Analysis complete. Outputs saved.");
    Ok(())
}

// Writing a HashMap<String, T> so T is displayable to a CSV file
// The arguments are path which is the outputting file path
// and data where the HashMap has every key being a label and value is a numerical result
// The output will have a csv that has two columns of a key, value
fn write_csv<T: std::fmt::Display>(path: &str, data: &std::collections::HashMap<String, T>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    writeln!(file, "key,value")?;
    for (k, v) in data {
        writeln!(file, "{} , {}", k, v)?;
    }
    Ok(())
}

// Writing each movie clusters to a plain text file, each cluster on its own line.
// the parameters are path which is the ouput txt file
// and clusters which is the vector of clusters that are each a vector of the movie titles
// I wanted the output to look like Cluster: movie, movie, ...
fn write_clusters(path: &str, clusters: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    for (i, cluster) in clusters.iter().enumerate() {
        writeln!(file, "Cluster {}: {}", i + 1, cluster.join(", "))?;
    }
    Ok(())
}