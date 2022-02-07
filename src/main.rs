use std::{env, fs};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
struct SynoContainerConfig {
    name: String,
    image: String,
    links: Vec<SynoContainerLink>,
    port_bindings: Vec<SynoContainerPortBinding>,
    volume_bindings: Vec<SynoContainerVolumeBinding>,
    env_variables: Vec<SynoContainerEnvVariable>
}

impl ToDocker for SynoContainerConfig {
    type Output = (String, DockerService);
    fn to_docker(&self) -> (String, DockerService) {
        let service = DockerService {
            image: self.image.clone(),
            depends_on: self.links.to_docker(),
            ports: self.port_bindings.to_docker(),
            volumes: self.volume_bindings.to_docker(),
            environment: self.env_variables.to_docker(),
        };
        (self.name.clone(), service)
    }
}

trait ToDocker {
    type Output;

    fn to_docker(&self) -> Self::Output;
}

#[derive(Deserialize, Debug)]
struct SynoContainerLink {
    //alias: String,
    link_container: String,
}

impl ToDocker for SynoContainerLink {
    type Output = String;
    fn to_docker(&self) -> String {
        self.link_container.clone()
    }
}

#[derive(Deserialize, Debug)]
struct SynoContainerPortBinding {
    container_port: i32,
    host_port: i32,
    #[serde(rename = "type")]
    port_type: String,
}

impl ToDocker for SynoContainerPortBinding {
    type Output = DockerPort;
    fn to_docker(&self) -> DockerPort {
        DockerPort {
            target: self.container_port,
            published: self.host_port,
            protocol: self.port_type.clone(),
            mode: "host".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct SynoContainerVolumeBinding {
    host_volume_file: String,
    mount_point: String,
    #[serde(rename = "type")]
    volume_type: String,
}

impl ToDocker for SynoContainerVolumeBinding {
    type Output = String;
    fn to_docker(&self) -> String {
        format!("{}:{}:{}", self.mount_point, self.host_volume_file, self.volume_type)
    }
}

#[derive(Deserialize, Debug)]
struct SynoContainerEnvVariable {
    key: String,
    value: String,
}

impl ToDocker for SynoContainerEnvVariable {
    type Output = String;
    fn to_docker(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

impl<T> ToDocker for Vec<T> where T: ToDocker {
    type Output = Vec<T::Output>;
    fn to_docker(&self) -> Self::Output {
        self.iter().map(|i| i.to_docker()).collect()
    }
}

impl ToDocker for BTreeMap<String, DockerService> {
    type Output = DockerCompose;
    fn to_docker(&self) -> DockerCompose {
        DockerCompose {
            version: "3.9".to_string(),
            services: (*self).clone(),
        }
    }
}

#[derive(Serialize, Debug)]
struct DockerCompose {
    version: String,
    services: BTreeMap<String, DockerService>,
}

#[derive(Serialize, Clone, Debug)]
struct DockerService {
    image: String,
    depends_on: Vec<String>,
    ports: Vec<DockerPort>,
    volumes: Vec<String>,
    environment: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
struct DockerPort {
    target: i32,
    published: i32,
    protocol: String,
    mode: String,
}

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

    let docker_config = configs.iter()
                               .map(|c| c.to_docker())
                               .collect::<BTreeMap<String, DockerService>>()
                               .to_docker();

    let output_file_name = "docker-compose.yml".to_string();
    let output_path = Path::new(output_path).join(output_file_name);
    println!("Outputing docker-compose.yml to {:?}", output_path);

    let output_file = fs::File::create(output_path).unwrap();
    let writer = BufWriter::new(output_file);

    serde_yaml::to_writer(writer, &docker_config)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}
