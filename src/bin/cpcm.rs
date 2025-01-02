use clap::Parser;

use cpcm::command_domain::run_domain;
use cpcm::command_server::run_server_add;
use cpcm::command_init::initialize;

use cpcm::cli::{
    Cpcm,
    ServerSubcommand,
};
use cpcm::global_paths::GlobalPaths;
use cpcm::config::Config;

#[tokio::main]
async fn main() {

    env_logger::init();

    log::debug!("Parsing arguments");
    let args = Cpcm::parse();

    log::debug!("Loading paths");
    let paths = match GlobalPaths::get_paths() {
        Ok(p) => p,
        Err(why) => panic!("{}", why)
    };

    log::debug!("Attempting to load configuration");
    let _config = Config::load(&paths);
    let config: Config;
    match _config {
        Ok(c) => {
            config = c;
            log::debug!("Config loaded.");
        },
        Err(why) => {
            log::debug!("Failed to load configuration!");
            match &args {
                Cpcm::Init(isc) => {
                    log::debug!("Found init command. Initializing");
                    config = initialize(isc, &paths).unwrap()
                },
                _ => panic!("Please use cpcm init to initialize default configuration. {}", why)
            }
        }
    };

    let r = match args {
        // Handle force init first
        Cpcm::Init(isc) => {
            log::debug!("Found init command. Initializing");
            match initialize(&isc, &paths) {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
            }
        },
        Cpcm::Domain( domargs ) => run_domain(domargs, &paths, &config).await,
        Cpcm::Server(subcmd) => match subcmd {
            ServerSubcommand::Add(s) => run_server_add(s, &paths, &config)
        },
    };

    match r {
        Err(e) => panic!("{:?}", e),
        _ => {}
    }

    ()
}
