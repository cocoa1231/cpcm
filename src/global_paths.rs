use std::{fs::create_dir_all, path::PathBuf};
use log::debug;
use std::error::Error;
use crate::error_types::{self, PathsError};


#[derive(Debug, Clone, PartialEq)]
pub enum PathType {
    Directory,
    File
}

#[derive(Debug, Clone)]
pub struct CfgPath {
    pub ptype: PathType,
    pub path: PathBuf
}

#[derive(Debug)]
pub struct GlobalPaths {
    pub cpcmdatadir: CfgPath,
    pub cpcmdbfile: CfgPath,
    pub cpcmlockfile: CfgPath,
    pub cpcmconfig: CfgPath,
}

impl GlobalPaths {
    pub fn as_array(&self) -> [&CfgPath; 3] {
        [&self.cpcmdatadir, &self.cpcmdbfile, &self.cpcmconfig]
    } 

    pub fn checkpaths(&self) -> Result<(), error_types::PathsError> {
        let missingpaths: Vec<&CfgPath> = self.as_array().into_iter()
            .filter(|&x| !x.path.exists())
            .collect();

        if missingpaths.is_empty() {
            Ok(())
        } else {
            Err(PathsError{ missingpaths: missingpaths })
        }


    }

    pub fn create_dirs(&self) -> Result<i32, Box<dyn Error>> {
        match self.checkpaths() {
            Ok(_) => Ok(0),
            Err(e) => {
                let mut i = 0;
                for path in e.missingpaths {
                    if path.ptype == PathType::Directory {
                        create_dir_all(path.path.clone())?;
                        i += 1;
                    }
                }
                Ok(i)
            }
        }

    }

    pub fn dbfile(&self) -> PathBuf {
        self.cpcmdbfile.path.clone()
    }
    pub fn get_paths() -> Result<Self, Box<dyn Error>> {
        let datadir = get_cpcm_datadir()?;
        let configfile = datadir.join("config.json");
        let lockfile = datadir.join(".cpcm-lock");
        let dbfile = datadir.join("cpcm.db");

        Ok(Self{
            cpcmdatadir: CfgPath{ptype: PathType::Directory, path: datadir},
            cpcmconfig: CfgPath{ptype: PathType::File, path: configfile},
            cpcmdbfile: CfgPath{ptype: PathType::File, path: dbfile},
            cpcmlockfile: CfgPath{ptype: PathType::File, path: lockfile}
        })
    }

    pub fn configfile(&self) -> &PathBuf {
        &self.cpcmconfig.path
    }

    pub fn datadir(&self) -> &PathBuf {
        &self.cpcmdatadir.path
    }
}

fn get_cpcm_datadir() -> Result<PathBuf, Box<dyn Error>> {
    match std::env::var("CPCM_DATA_DIR") {
        Ok(datadir) => {
            let datadir = datadir.trim();

            if datadir.is_empty() {
                return get_cpcm_default_datadir()
            } else {
                let rval = PathBuf::from(datadir);
                if rval.is_relative() {
                    Err( format!("Cannot use {} as data dir, found relative path.", rval.display()) )?
                } else {
                    Ok(rval)
                }
            }
        }
        Err(e) => {
            let datadir = get_cpcm_default_datadir()?;

            debug!("{}. Using default {}", e, datadir.display() );

            Ok(datadir)
        }
    }
}

fn get_cpcm_default_datadir() -> Result<PathBuf, Box<dyn Error>> {
    let path = dirs::home_dir()
        .ok_or_else(|| format!("Unable to find user's home directory"))?
        .join(".cpcm");

    if !path.is_absolute() {
        return Err(format!("System did not return a valid home directory path {}", path.display())
            .into())
    } else {
        return Ok(path)
    }
}

