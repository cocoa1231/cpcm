use crate::global_paths::GlobalPaths;
use crate::config::Config;
use crate::sql_strings::SQLSCHEMA;
use crate::cli::InitSubcommand;

use std::{
    fs,
    io,
    error::Error
};

#[derive(Clone, Debug)]
pub struct InitError;

pub fn initialize(args: &InitSubcommand, paths: &GlobalPaths) -> Result<Config, Box<dyn Error>> {
    log::debug!("args: {:?}\npaths: {:?}", &args, &paths);

    if args.force {
        println!("You are about to delete your database along with all your config files!!!");
        println!("Continue? [y/N] ");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let ioout = match buf.trim() {
            "Yes" | "Y" | "y" => {
                fs::remove_dir_all(paths.datadir())?;
                Ok(())
            }
            _ => Err(InitError)
        };

        if ioout.is_err() {
            std::process::exit(1);
        }
    } else {
        if fs::exists(paths.datadir())? && fs::exists(paths.configfile())? {
            println!("Configuration already exists. Please use --force to delete and reset.");
            let cfg = Config::load(paths);

            return cfg;
        }
    }

    log::debug!("Creating configuration directories");
    paths.create_dirs()?;

    // Creating default config
    let defaultcfg = Config::default();

    log::debug!("Creating SQLite database");
    // Create SQLite database;
    let dbconn = rusqlite::Connection::open(paths.dbfile())?;
    let _ = SQLSCHEMA(&defaultcfg).split("-- Statement\n").filter(|&x| !x.is_empty())
        .into_iter()
        .inspect(|s| log::debug!("Running {}", s))
        .map(|s| {
            let r = dbconn.execute(s, rusqlite::params![]);
            match r {
                Ok(x)  => log::debug!("Command succeded with code {}", x),
                Err(x) => log::error!("{:?}", x)
            }
        })
        .collect::<Vec<_>>();



    defaultcfg.write_file(paths)?;

    Ok(defaultcfg)
}
