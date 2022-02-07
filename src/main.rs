use std::{env, fs};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Error, ErrorKind};

mod syno_container;
mod docker;
mod to_docker;

use syno_container::SynoContainerConfig;
use to_docker::ToDocker;

fn is_syno_docker_json_file(path: &Path) -> bool {
    let file_name = path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or_else(|| {
            println!("Unable to determine filename for {:?}", path);
            ""
        });

    file_name.ends_with(".syno.json")
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    let output_path = &args[2];

    println!("Finding syno.json files in {}", input_path);

    let path = Path::new(input_path);
    if !path.is_dir() {
        return Err(Error::new(ErrorKind::Other, format!("{:?} is not a directory", path)));
    }

    let entries = fs::read_dir(path).unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| is_syno_docker_json_file(p.as_ref()))
        .collect::<Vec<PathBuf>>();

    if entries.len() == 0 {
        return Err(Error::new(ErrorKind::NotFound, format!("{:?} contains no syno docker json files", path)));
    }

    println!("Found:");
    for item in &entries {
        println!("\t{:?}", item);
    }

    let configs = entries.iter().map(|p| {
        let file = fs::File::open(p).unwrap();
        let reader = BufReader::new(file);
        let config: SynoContainerConfig = serde_json::from_reader(reader).unwrap();
        config
    }).collect::<Vec<SynoContainerConfig>>();

    let docker_config = configs.iter().to_docker();

    let output_file_name = "docker-compose.yml".to_string();
    let output_path = Path::new(output_path).join(output_file_name);
    println!("Outputing docker-compose.yml to {:?}", output_path);

    let output_file = fs::File::create(output_path).unwrap();
    let writer = BufWriter::new(output_file);

    serde_yaml::to_writer(writer, &docker_config)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}
