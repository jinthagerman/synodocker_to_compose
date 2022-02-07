use std::collections::BTreeMap;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct DockerCompose {
    pub version: String,
    pub services: BTreeMap<String, DockerService>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DockerService {
    pub image: String,
    pub depends_on: Vec<String>,
    pub ports: Vec<DockerPort>,
    pub volumes: Vec<String>,
    pub environment: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DockerPort {
    pub target: i32,
    pub published: i32,
    pub protocol: String,
    pub mode: String,
}