mod db;
mod logger;
mod models;
mod routes;
//mod schema;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    logger::init()?;

    Ok(())
}