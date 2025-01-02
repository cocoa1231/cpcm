use std::fmt;
use crate::global_paths::CfgPath; 

#[derive(Debug, Clone)]
pub struct PathsError<'a> {
    pub missingpaths: Vec<&'a CfgPath>
}

impl fmt::Display for PathsError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Following paths do not exist.")?;
        self.missingpaths.clone().into_iter()
            .for_each(|p| write!(f, "- {}", p.path.display()).unwrap());
        write!(f, "")
    }
}
