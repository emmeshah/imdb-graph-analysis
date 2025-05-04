// Data Cleaning
// Purpose: Load and clean IMDb dataset, compute score differences, and prepare structured data for graph analysis.

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use serde::Deserialize;

// A struct for each movie from rows of CSV
// Only including the relevant fields needed for the future analysis
#[derive(Debug, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: String,
    pub genre: String,
    pub metacritic: Option<f64>,
    pub imdbRating: Option<f64>,
}

// A cleaner version of the data that hincludes score gap and parsed year
#[derive(Debug, Clone)]
pub struct CleanMovie {
    pub title: String,
    pub year: u16,
    pub genres: Vec<String>,
    pub critic_score: f64,
    pub user_score: f64,
    pub score_gap: f64,
}

// Reading the dataset from CSV file and returns a vector of cleaned movies.
// Filtering out entries missing scores or invalid years.
pub fn load_and_clean_data(path: &str) -> Result<Vec<CleanMovie>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut clean_movies = Vec::new();

    for result in reader.deserialize() {
        let record: Movie = result?;

        // Filtering rows missing essential data
        if let (Some(critic), Some(user)) = (record.metacritic, record.imdbRating) {
            // Normalizing the Metacritic score to adapt to a 0–10 scale
            let critic_score = critic / 10.0;
            let user_score = user;
            let score_gap = user_score - critic_score;

            // Parsing year
            if let Ok(parsed_year) = record.year[..4].parse::<u16>() {
                // Splitting genres by comma and trimming whitespace
                let genres: Vec<String> = record
                    .genre
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                clean_movies.push(CleanMovie {
                    title: record.title,
                    year: parsed_year,
                    genres,
                    critic_score,
                    user_score,
                    score_gap,
                });
            }
        }
    }

    Ok(clean_movies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleaning_sample() {
        let sample_path = "./test_data/sample.csv";
        let result = load_and_clean_data(sample_path);
        assert!(result.is_ok());
        let movies = result.unwrap();
        assert!(!movies.is_empty());
        for movie in movies {
            assert!(movie.critic_score >= 0.0 && movie.critic_score <= 10.0);
            assert!(movie.user_score >= 0.0 && movie.user_score <= 10.0);
            assert!(movie.year >= 1880);
            assert!(!movie.genres.is_empty());
        }
    }
} 
