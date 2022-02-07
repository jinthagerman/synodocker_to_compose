use std::collections::BTreeMap;
use std::slice::Iter;
use crate::syno_container;
use crate::docker;

pub trait ToDocker {
    type Output;

    fn to_docker(&self) -> Self::Output;
}

impl ToDocker for syno_container::SynoContainerConfig {
    type Output = (String, docker::DockerService);
    fn to_docker(&self) -> (String, docker::DockerService) {
        let service = docker::DockerService {
            image: self.image.clone(),
            depends_on: self.links.to_docker(),
            ports: self.port_bindings.to_docker(),
            volumes: self.volume_bindings.to_docker(),
            environment: self.env_variables.to_docker(),
        };
        (self.name.clone(), service)
    }
}

impl ToDocker for syno_container::SynoContainerLink {
    type Output = String;
    fn to_docker(&self) -> String {
        self.link_container.clone()
    }
}

impl ToDocker for syno_container::SynoContainerPortBinding {
    type Output = docker::DockerPort;
    fn to_docker(&self) -> docker::DockerPort {
        docker::DockerPort {
            target: self.container_port,
            published: self.host_port,
            protocol: self.port_type.clone(),
            mode: "host".to_string(),
        }
    }
}

impl ToDocker for syno_container::SynoContainerVolumeBinding {
    type Output = String;
    fn to_docker(&self) -> String {
        format!("{}:{}:{}", self.mount_point, self.host_volume_file, self.volume_type)
    }
}

impl ToDocker for syno_container::SynoContainerEnvVariable {
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

impl ToDocker for Iter<'_, syno_container::SynoContainerConfig> {
    type Output = docker::DockerCompose;
    fn to_docker(&self) -> docker::DockerCompose {
        (*self).clone()
               .map(|c| c.to_docker())
               .collect::<BTreeMap<String, docker::DockerService>>()
               .to_docker()
    }
}

impl ToDocker for BTreeMap<String, docker::DockerService> {
    type Output = docker::DockerCompose;
    fn to_docker(&self) -> docker::DockerCompose {
        docker::DockerCompose {
            version: "3.9".to_string(),
            services: (*self).clone(),
        }
    }
}