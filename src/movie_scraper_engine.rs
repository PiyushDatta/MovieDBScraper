/**
 * Main engine to scrape movie data from the internet.
 */
pub mod movie_scraper_engine {
    struct MovieScraperEngine {
        movie_sites: Vec<String>,
    }

    impl MovieScraperEngine {
        pub fn new() -> MovieScraperEngine {
            return MovieScraperEngine {
                movie_sites: Vec<String>::new()
            }
        }

        pub fn start_scraping(self: Self) -> () {
            println!("Vector: {:?}", self.movie_sites);
            return;
        }
    }
}