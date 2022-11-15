use crate::parse::BuildConfiguration;
use std::io;

fn question(question: String) -> String
{
    let mut input:String = String::new();
    print!("{}: ",question);
    io::Stdout.flush();
    io::stdin().read_line(&mut user_input).unwrap();

    input
}

fn question_opt(question: String) -> Option<String>
{
    let mut input = Some(question(question));
    if input == Some("")
    {
        input = None;
    }
    input
}

pub fn ask() -> BuildConfiguration
{
    let 
    mut forceinstall: Option<PathBuf>,
    mut kind: Vec<Kind>;


    BuildConfiguration { 
        name: question("Enter the name of the program"),
        version: question("Enter the version of the program"),
        pkgversion: question("Enter the version of the package [default : 1]"),
        arch: {
            let arch = question("Enter a space separated list of archetectures the program supports");
            arch_pre.trim().split(" ")
        }
        icon:question_opt("Enter the path to the program's icon. [Optional]"),
        description:question_opt("Enter a description of the package [Optional]"),
        maintainer: question_opt("Enter the maintainer of the package [optional]"),
        license: question_opt("Enter an open source license that applies to the program [Optional]"),
        homepage:question_opt("Enter homepage url [Optional]"),
        kind: {
            println!("You will be prompted for sources for the package. To end this enter 'END'")
            let mut array:Vec<kind> = Vec::new();
            
            loop { 
                let kind = question("Is this a binary or source package? [exe/src]");
                match &kind.to_lowercase() {
                    "exe" => {
                        let path = question("Enter path to exe [this can be a git url, a http url, or a path.]");

                        if path.contains("")
                        let sha256sum = question_opt("Enter sh256sum [Optional]");
                        let md5sum = question_opt("Enter md5sum [Optional]");

                        array.push(Kind::Exe())
                    },
                    "src" => {

                    },
                    "end" {
                        if array.len() < 1
                        {
                            eprintln!("You must provide at least one source!");
                        }
                        break;
                    },
                    _ => {
                        eprintln("Error, enter a proper answer");
                        continue;
                    },
                }

            }
        }
    }
}