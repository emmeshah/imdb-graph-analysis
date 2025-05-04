// Data Cleaning
// Purpose: I want to load and clean the IMDb dataset
// Then I want to compute the score differences
// Finally, I want to return the structured movie data for analysis later

use std::error::Error; // Using this to return error types from the functions
use std::fs::File; // Using this to open the CSV file
use std::io::BufReader; // Using this because the file is really large and requires efficient reading of it
use csv::ReaderBuilder; // This is a CSV reader
use serde::Deserialize; // Using this for CSV parsing from struct 

// A struct for each movie from rows of CSV
// Only including the relevant fields needed for the future analysis
// Fields:
// title: Movie title is a string
// year: Year of the movie's release as a string
// genre: Comma separated genre list as a string
// metacritic: The critic score that is on a scale of 0 to 100
// imdbRating: The IMDb user score that is  on a scale of 0 to 10
#[derive(Debug, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: String,
    pub genre: String,
    pub metacritic: Option<f64>,
    pub imdbRating: Option<f64>,
}

// This struct is for a cleaner version of the data that hincludes score gap and parsed year
// It's just like a filtered version of the Movie struct
// Fields:
// title: Movie title as a string
// year: parsed year as a u16 instead of string
// genre: vector of genre instead of string
// critic score: The critic score is now on a scale of 0 to 10
// user_score: The IMDb score
// score gap: This will be the difference of user and critic score
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
// The arguments here is path which is the path to the CSV file and I'm making it as a string
// It will return a vector of CleanMovie structs 
// It will read the CSV line by line, filtering out missing required fields, normalize the scores
// on a scale of 0 to 10. Then parse years into integers and then split genres into a vector.
pub fn load_and_clean_data(path: &str) -> Result<Vec<CleanMovie>, Box<dyn Error>> {
    let file = File::open(path)?; //opens the file for reading
    let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file)); //using the buffered reader for efficiency
    let mut clean_movies = Vec::new(); // a vector to store cleaned movie entries

    for result in reader.deserialize() { // going to iterate through every row in the csv file
        let record: Movie = result?; // to deserialize this row into a movie struvt

        // filtering rows missing essential data
        if let (Some(critic), Some(user)) = (record.metacritic, record.imdbRating) { // checking both critic and user scores are present
            // normalizing the Metacritic score to adapt to a 0–10 scale
            let critic_score = critic / 10.0;
            let user_score = user;
            let score_gap = user_score - critic_score;

            // parsing year
            if let Ok(parsed_year) = record.year[..4].parse::<u16>() { //extracting first 4 characters of the year string to parse as a number
                // splitting genres by comma and trimming whitespace
                let genres: Vec<String> = record
                    .genre
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                // adding the cleaned data to a result list
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

    Ok(clean_movies) // return the vector of cleaned and valid movie entries
}

#[cfg(test)] // unit testing verifying the cleaning logistics works on a sample file
mod tests {
    use super::*;

    #[test]
    fn test_cleaning_sample() {
        let sample_path = "./test_data/sample.csv"; // test input file
        let result = load_and_clean_data(sample_path); // running the cleaning function on the sample csv
        assert!(result.is_ok()); // did we get a successful result?
        let movies = result.unwrap(); // unwrapping the result to get the movie list
        assert!(!movies.is_empty()); // is the list empty?
        for movie in movies { // checking properties of each movie
            assert!(movie.critic_score >= 0.0 && movie.critic_score <= 10.0);
            assert!(movie.user_score >= 0.0 && movie.user_score <= 10.0);
            assert!(movie.year >= 1880);
            assert!(!movie.genres.is_empty());
        }
    }
} 
