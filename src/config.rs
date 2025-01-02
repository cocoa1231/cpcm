use std::{error::Error, io::Read, fs};
use serde::{Serialize, Deserialize};
use crate::global_paths::GlobalPaths;


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub tabname_domain: Option<String>,
    pub tabname_server: Option<String>
}

impl Config {
    pub fn load(paths: &GlobalPaths) -> Result<Config, Box<dyn Error>> {
        let mut file = fs::File::open(paths.configfile())?;
        let mut jsoncfg = String::new();
        file.read_to_string(&mut jsoncfg)?;

        let mut config: Self = serde_json::from_str(&jsoncfg)?;

        // Defaults
        config.tabname_domain = match config.tabname_domain {
            Some(s) => Some(s),
            None => Some("domains".to_string())
        };
        config.tabname_server = match config.tabname_server {
            Some(s) => Some(s),
            None => Some("servers".to_string())
        };

        Ok(config)
    }

    // motherfucking monads
    pub fn tabname_domain(&self) -> &String {
        // i just realized moands sounds like gonads
        self.tabname_domain.as_ref().unwrap()
    }

    pub fn tabname_server(&self) -> &String {
        // i just realized moands sounds like gonads
        self.tabname_server.as_ref().unwrap()
    }
    pub fn write_file(&self, paths: &GlobalPaths) -> Result<(), Box<dyn Error>> {
        let json_data = serde_json::to_string(self)?;
        fs::write(paths.configfile(), json_data)?;

        Ok(())
    }


    pub fn default() -> Self {
        Self {
            tabname_domain: Some("domains".to_string()),
            tabname_server: Some("servers".to_string())
        }
    }

}
