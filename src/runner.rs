use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::{
    env::current_dir,
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use toml::from_str;

#[derive(Clone, Debug)]
pub struct Runner {
    project: Project,
}

impl Runner {
    pub fn new(project: Project) -> Runner {
        Runner { project }
    }
}

#[derive(Clone, Debug)]
pub struct Project {
    pub path: PathBuf,
    version: Version,
}

impl Project {
    pub fn try_new(path: &Path) -> Result<Project, Box<dyn Error>> {
        let version = Project::version(&path)?;
        Ok(Project {
            path: path.to_owned(),
            version,
        })
    }

    pub fn version(path: &Path) -> Result<Version, Box<dyn Error>> {
        let path = current_dir()?.join("Cargo.toml");
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Manifest = from_str(&contents)?;
        Ok(config.package.version)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    pub name: String,
    pub version: Version,
    pub authors: Vec<String>,
    pub edition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Manifest {
    pub package: Config,
}
