/**
 * Client for our Movie Scraper.
 */
pub mod movie_scraper_client {
    use config::Config;
    use rustyline::error::ReadlineError;
    use rustyline::DefaultEditor;
    use std::collections::HashMap;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::Duration;
    use tracing::{error, info};

    pub struct MovieScraperClient {
        movie_scraper_engine: MovieScraperEngine,
    }

    impl MovieScraperClient {
        pub fn new(movie_scraper_eng: MovieScraperEngine) -> MovieScraperClient {
            MovieScraperClient {
                movie_scraper_engine: movie_scraper_eng,
            }
        }

        pub async fn run(&mut self) -> Result<(), ReadlineError> {
            let mut rl = DefaultEditor::new()?;
            loop {
                let readline = rl.readline("\n>> ");
                match readline {
                    Ok(line) => match &line as &str {
                        // start movie scraper engine
                        "start"
                        | "start_movie_scraper"
                        | "start_movie_scraping"
                        | "start scraping" => self.start_scraping().await,
                        // TODO(PiyushDatta): Implement status of scraper
                        // show status
                        // "status" | "show_server_status" => self.show_server_status().await,
                        "" => (),
                        "exit" | "quit" | "exit()" | "quit()" => {
                            println!("Exiting client.");
                            break;
                        }
                        _ => println!("Unknown command."),
                    },
                    Err(ReadlineError::Interrupted) => {
                        println!("Exiting client. CTRL-C pressed.");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("Exiting client. Eof/CTRL-D pressed.");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
            Ok(())
        }

        async fn start_scraping(&mut self) -> () {
            println!("Starting movie scraper engine...");
            self.movie_scraper_engine
        }
    }
}
