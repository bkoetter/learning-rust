use std::env;
use std::error::Error;
use reqwest::blocking::get;

#[derive(Debug)]
struct Config {
    api_url: url::Url,
    _api_key: String,
    _username: String,
}

impl Config {
    fn new() -> Result<Config, &'static str> {
        Ok(Config {
            api_url: url::Url::parse("https://fake-json-api.mock.beeceptor.com/users")
                .map_err(|_| "Invalid URL")?,
            _api_key: env::var("API_KEY")
                .map_err(|_| "Environment variable 'API_KEY' not set")?,
            _username: env::var("API_USER")
                .map_err(|_| "Environment variable 'API_USER' not set")?,
        })
    }
}

fn get_api_data(config: Config) -> String {
    get(config.api_url).unwrap().text().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;
    println!("{:?}", get_api_data(config));
    Ok(())
}
