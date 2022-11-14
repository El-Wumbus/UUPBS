use progress_bar::*;
use std::path::PathBuf;
use serde_derive::Deserialize;
use serde::Serialize;

pub fn parse_build_file(file:PathBuf, verbose:u8) -> Result<BuildConfiguration, String>
{
    init_progress_bar(1);
    set_progress_bar_action("Reading build file", Color::Blue, Style::Bold);

    let read:String = match read_to_string(file.clone())
    {
        Ok(x) => x,
        Err(x) => print_progress_bar_info(
            "Failed",
            format!("reading from file '{}'", file.display()),
            Color::Red,
            Style::Normal
        );
        return Err(format!("Error: Couldn't read from build file: {}", x)),
    };
    
    if verbose > 0 {
        print_progress_bar_info(
            "Success",
            format!("read from file '{}'",
            file.display()),
            Color::Green,
            Style::Normal
        );
    }
    

    let parsed:Config = match toml::from_str(&read){
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    if verbose > 1{
        print_progress_bar_info(
            "Success",
            format!("parsed info from file '{}'",
            file.display()),
            Color::Green,
            Style::Normal
        );
    }

    if verbose >= 3{
        print_progress_bar_info(
            "Success",
            format!("read from file '{}'",
            file.display()),
            Color::Green,
            Style::Normal
        );

        print_progress_bar_info("Info", format!("{:#?}", parsed), Color::Normal, Color::Normal)
    }

    Ok(parsed)
}

#[derive(Deserialize, Serialize, debug)]
pub struct BuildConfiguration
{
    pub name: String,
    pub icon: Option<PathBuf>,
    pub version: String,
    pub pkgversion: u8,
    pub maintainer: Option<String>,
    pub description: Option<String>,
    pub arch: Vec<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub forceinstall: Option<PathBuf>,
    pub kind: Kind,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "source")]
pub enum Kind {
    pub Exe(Vec<Exe>),
    pub Src(Vec<SourceKind>),
}

#[derive(debug, Deserialize, Serialize)]
pub struct Exe {
    pub path: PathBuf,
    pub sha256sum: Option<String>,
    pub md5sum: Option<String>,
}

#[derive(debug, Deserialize, Serialize)]
pub struct SourceKind {
    /// A file or folder where the build system is (supported are meson, make, cargo, and, cmake)
    pub location: LocationType,
    pub builder: String,

    /// The file or folder the build system outputs the binaries to. 
    pub builder_output: PathBuf,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "source")]
pub enum LocationType
{
    pub Git(String),
    pub Path(pathbuf),
}