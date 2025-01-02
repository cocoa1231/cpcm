use clap::Args;
use std::error::Error;
use crate::global_paths::GlobalPaths;
use crate::config::Config;
use crate::sql_strings::SERVERADD_UPSERT;
use rusqlite::{params, Connection};
#[derive(Debug, Args)]
pub struct ServerAdd {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    ip: String,
    #[arg(short, long)]
    user: String,
    #[arg(long)]
    hostname: Option<String>,
    #[arg(short, long)]
    group: Option<String>
}


pub fn run_server_add(server: ServerAdd, paths: &GlobalPaths, config: &Config) -> Result<(), Box<dyn Error>> {

    log::debug!("Got server {:?}", &server);
    // Prompt user for api key
    let apikey = rpassword::prompt_password("API Key: ")?;

    let db = Connection::open(paths.cpcmdbfile.path.clone())?;
    let mut stmt = db.prepare(&SERVERADD_UPSERT(config))?;

    log::debug!("Running {:?}", stmt);
    stmt.execute(params![
        server.name, server.ip, server.user, apikey.trim(),
        server.hostname.unwrap_or("NULL".to_string()),
        server.group.unwrap_or("NULL".to_string())
    ])?;
    
    Ok(())
}
